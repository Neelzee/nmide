use async_trait::async_trait;
use core_std_lib::{
    core::Core,
    core_modification::CoreModification,
    event::Event,
    state::{Value, state_builder::StateBuilder},
};
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};
use std::{collections::HashSet, fs};

#[cfg(test)]
mod tests;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

const MODULE_NAME: &str = "rust_dependency";

#[async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler("get_graph".to_string(), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        if event.event_name() == "get_graph" {
            let path: PathBuf = core
                .state()
                .await
                .get("ide-cache.project")
                .and_then(|v| v.str())
                .unwrap_or_default()
                .into();

            if !is_rust_project(Path::new(&path)) {
                return;
            }

            let field = format!("graph:{path:?}");
            match core.state().await.get(&field) {
                Some(g) => {
                    core.throw_event(Event::new("graph", Some(g.clone()))).await;
                }
                None => {
                    let graph = get_graph(&path);
                    core.throw_event(Event::new("graph", Some(graph.clone())))
                        .await;
                    let mods = CoreModification::default()
                        .set_state(StateBuilder::default().add(field, graph));
                    core.send_modification(mods).await;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RustModule {
    #[allow(dead_code, reason = "Handy to have the path for debugging purposes")]
    pub path: String,
    pub name: String,
    pub dependencies: Vec<String>,
}

impl RustModule {
    pub fn new(path: &Path) -> Self {
        let use_regex =
            Regex::new(r"^(?:pub(?:\(crate\))?\s+)?use\s+((?:crate|self|super)(?:::\w+)+[^;]*);")
                .unwrap();

        let module_regex = Regex::new(r"(?:crate|self|super)(?:::([a-z_][a-z0-9_]*)+)+").unwrap();

        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut dependencies_set = HashSet::new();

        for line in contents.lines() {
            if let Some(captures) = use_regex.captures(line) {
                if let Some(use_path) = captures.get(1) {
                    let path_str = use_path.as_str();

                    for module_capture in module_regex.captures_iter(path_str) {
                        for i in 1..module_capture.len() {
                            if let Some(module_name) = module_capture.get(i) {
                                let module_str = module_name.as_str();
                                dependencies_set.insert(module_str.to_string());
                            }
                        }
                    }
                }
            }
        }

        Self {
            path: path.to_string_lossy().to_string(),
            name,
            dependencies: dependencies_set.into_iter().collect(),
        }
    }

    pub fn to_obj(&self) -> Value {
        let mut mp = HashMap::new();
        mp.insert("name".to_string(), Value::Str(self.name.clone()));
        mp.insert(
            "dependencies".to_string(),
            Value::List(
                self.dependencies
                    .clone()
                    .into_iter()
                    .map(Value::Str)
                    .collect::<Vec<Value>>(),
            ),
        );

        mp.into()
    }
}

pub(crate) fn get_graph(path: &PathBuf) -> Value {
    Value::List(
        get_modules(Path::new(path))
            .iter()
            .map(RustModule::to_obj)
            .collect::<Vec<Value>>(),
    )
}
pub(crate) fn get_modules(path: &Path) -> Vec<RustModule> {
    let modules: Vec<RustModule> = fs::read_dir(
        path.canonicalize()
            .inspect_err(|err| panic!("Error when canonicalizing path: {path:?}, error: {err:?}"))
            .unwrap(),
    )
    .inspect_err(|err| panic!("Error when reading path: {path:?}, error: {err:?}"))
    .unwrap()
    .filter_map(|e| e.ok())
    .flat_map(|d| match d.file_type() {
        Ok(df) if df.is_file() && d.file_name().to_str().is_some_and(|p| p.ends_with(".rs")) => {
            vec![RustModule::new(&d.path())]
        }
        Ok(df) if df.is_dir() => get_modules(&d.path()),
        _ => Vec::new(),
    })
    .collect();

    let modules = modules
        .into_iter()
        .map(|m| {
            if m.name == "mod" {
                let path = Path::new(&m.path);
                if let Some(parent) = path.parent() {
                    if let Some(parent_name) = parent.file_name() {
                        if let Some(parent_str) = parent_name.to_str() {
                            return RustModule {
                                name: parent_str.to_string(),
                                ..m
                            };
                        }
                    }
                }
                m
            } else {
                m
            }
        })
        .collect::<Vec<RustModule>>();

    let mut module_map: HashMap<String, RustModule> = HashMap::new();

    for module in modules {
        match module_map.get(&module.name) {
            Some(old_module) if old_module.path == module.path => {
                let mut deps: HashSet<String> = HashSet::new();
                let old_deps: Vec<String> = old_module.dependencies.clone();
                deps.extend(old_deps);
                deps.extend(module.dependencies);

                module_map.insert(
                    module.name.clone(),
                    RustModule {
                        dependencies: deps.into_iter().collect(),
                        ..module
                    },
                );
            }
            _ => {
                module_map.insert(module.name.clone(), module);
            }
        }
    }

    module_map.values().cloned().collect()
}

fn is_rust_project(path: &Path) -> bool {
    fs::read_dir(
        path.canonicalize()
            .inspect_err(|err| panic!("Error when canonicalizing path: {path:?}, error: {err:?}"))
            .unwrap(),
    )
    .inspect_err(|err| panic!("Error when reading path: {path:?}, error: {err:?}"))
    .unwrap()
    .filter_map(|e| e.ok())
    .any(|d| match d.file_type() {
        Ok(df) if df.is_file() && d.file_name().to_str().is_some_and(|p| p.ends_with(".rs")) => {
            true
        }
        Ok(df) if df.is_dir() => is_rust_project(&d.path()),
        _ => false,
    })
}
