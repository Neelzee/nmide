use std::collections::HashMap;
use std::fs;
use async_trait::async_trait;
use core_std_lib::{
    attrs::Attr,
    core::{Core, CoreModification},
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{StateInstructionBuilder, Value},
};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use regex::Regex;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

const MODULE_NAME: &'static str = "magnolia_dependency";

#[async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        "trivial module"
    }

    async fn init(&self, core: &dyn Core) -> CoreModification {
        core.add_handler(Some("get_magnolia_graph".to_string()), None, MODULE_NAME.to_string())
            .await;
        CoreModification::default()
    }

    async fn handler(&self, event: &Event, _: &dyn Core) -> CoreModification {
        match (event.event_name(), event.args()) {
            ("get_magnolia_graph", Some(Value::Str(path))) => {
                let graph = get_graph(path);
                let state = StateInstructionBuilder::default().add("magnolia_graph", graph);
                CoreModification::default().set_state(state)
            }
            _ => CoreModification::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MagnoliaModule {
    path: String,
    name: String,
    dependencies: Vec<String>,
}

impl MagnoliaModule {
    pub fn new(path: &Path) -> Self {
        let re = Regex::new(r"package\s*([\w.]+)(?:\s*//.*?)?(?:\s*imports\s*([\s\S]*?))?;").unwrap();

        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut name = String::new();
        let mut dependencies = Vec::new();
        if let Some(caps) = re.captures(&contents) {
            let mut caps = caps.iter();
            caps.next();
            name = caps.next().unwrap().unwrap().as_str().to_string();
            dependencies = caps.next()
                .and_then(|p| p)
                .and_then(|p| {
                    Some(
                        p.as_str()
                            .split(",")
                            .map(|s| s.trim())
                            .map(|s| s.to_string())
                            .filter(|s| !s.contains("//"))
                            .collect()
                    )
                }).unwrap_or(Vec::new());
        }

        Self {
            path: path.to_str().unwrap_or_default().to_string(),
            name,
            dependencies: Vec::new(),
        }
    }

    pub fn to_obj(self) -> Value {
        let mut mp = HashMap::new();
        mp.insert("name".to_string(), Value::Str(self.name));
        mp.insert("dependencies".to_string(), Value::Str(self.dependencies.into_iter().map(Value::Str).collect()));

        Value::Obj(mp)
    }
}

pub(crate) fn get_graph(path: &str) -> Value {
    get_modules(Path::new(path)).into_iter().map(MagnoliaModule::to_obj).collect()
}

pub(crate) fn get_modules(path: &Path) -> Vec<MagnoliaModule> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .flat_map(|d| {
            match d.file_type() {
                Ok(df) if df.is_file() && d.file_name().to_str().is_some_and(|p| p.ends_with(".mg")) => vec![MagnoliaModule::new(&d.path())],
                Ok(df) if df.is_dir() => get_modules(&d.path()),
                _ => Vec::new(),
            }
        })
        .collect()
}

