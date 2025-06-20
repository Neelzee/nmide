use async_trait::async_trait;
use core_std_lib::{
    core::Core,
    core_modification::CoreModification,
    event::Event,
    state::{Value, state_builder::StateBuilder},
};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

const MODULE_NAME: &str = "magnolia_dependency";

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

            if !is_magnolia_project(Path::new(&path)) {
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

#[derive(Debug)]
pub(crate) struct MagnoliaModule {
    #[allow(dead_code, reason = "Handy to have the path for debugging purposes")]
    pub path: String,
    pub name: String,
    pub dependencies: Vec<String>,
}

impl MagnoliaModule {
    pub fn new(path: &Path) -> Self {
        let re =
            Regex::new(r"package\s*([\w.]+)(?:\s*//.*?)?(?:\s*imports\s*([\s\S]*?))?;").unwrap();

        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut name = String::new();
        let mut dependencies = Vec::new();
        if let Some(caps) = re.captures(&contents) {
            let mut caps = caps.iter();
            caps.next();
            name = caps.next().unwrap().unwrap().as_str().to_string();
            dependencies = caps
                .next()
                .and_then(|p| p)
                .map(|m| m.as_str().to_string())
                .map(|p| {
                    p.trim()
                        .split(",")
                        .map(|s| s.trim())
                        .map(|s| s.to_string())
                        .filter(|s| !s.contains("//"))
                        .collect()
                })
                .unwrap_or_default();
        }

        Self {
            path: path.to_str().unwrap_or_default().to_string(),
            name,
            dependencies,
        }
    }

    pub fn to_obj(self) -> Value {
        let mut mp = HashMap::new();
        mp.insert("name".to_string(), Value::Str(self.name));
        mp.insert(
            "dependencies".to_string(),
            Value::List(
                self.dependencies
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
            .into_iter()
            .map(MagnoliaModule::to_obj)
            .collect::<Vec<Value>>(),
    )
}

pub(crate) fn get_modules(path: &Path) -> Vec<MagnoliaModule> {
    fs::read_dir(
        path.canonicalize()
            .inspect_err(|err| panic!("Error when canonicalizing path: {path:?}, error: {err:?}"))
            .unwrap(),
    )
    .inspect_err(|err| panic!("Error when reading path: {path:?}, error: {err:?}"))
    .unwrap()
    .filter_map(|e| e.ok())
    .flat_map(|d| match d.file_type() {
        Ok(df) if df.is_file() && d.file_name().to_str().is_some_and(|p| p.ends_with(".mg")) => {
            vec![MagnoliaModule::new(&d.path())]
        }
        Ok(df) if df.is_dir() => get_modules(&d.path()),
        _ => Vec::new(),
    })
    .collect()
}

fn is_magnolia_project(path: &Path) -> bool {
    fs::read_dir(
        path.canonicalize()
            .inspect_err(|err| panic!("Error when canonicalizing path: {path:?}, error: {err:?}"))
            .unwrap(),
    )
    .inspect_err(|err| panic!("Error when reading path: {path:?}, error: {err:?}"))
    .unwrap()
    .filter_map(|e| e.ok())
    .any(|d| match d.file_type() {
        Ok(df) if df.is_file() && d.file_name().to_str().is_some_and(|p| p.ends_with(".mg")) => {
            true
        }
        Ok(df) if df.is_dir() => is_magnolia_project(&d.path()),
        _ => false,
    })
}
