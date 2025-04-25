use async_trait::async_trait;
use core_std_lib::{
    attrs::Attr,
    core::{Core},
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{StateInstructionBuilder, Value},
};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

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
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(
            Some("get_magnolia_graph".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        let mods =
            CoreModification::default().set_ui(
                UIInstructionBuilder::default().add_node(
                    Html::Div {
                        kids: vec![Html::Button().set_text("Graph").add_attr(Attr::Click(
                            Event::new(
                                "get_magnolia_graph".to_string(),
                                MODULE_NAME.to_string(),
                                Some(Value::Str(
                                    "/home/nmf/magnolia-basic-library/src".to_string(),
                                )),
                            ),
                        ))],
                        attrs: vec![Attr::Id("graph-btn-div".to_string())],
                        text: None,
                    },
                    None,
                    None,
                ),
            );
        core.get_sender()
            .await
            .send(mods)
            .await
            .expect("Channel should be opened");
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        let sender = core.get_sender().await;
        match (event.event_name(), event.args()) {
            ("get_magnolia_graph", Some(val))
                if val.is_str() || val.obj().is_some_and(|o| o.contains_key("eventArgs")) =>
            {
                let path = if val.is_str() {
                    val.str().unwrap()
                } else {
                    let p = val.obj().unwrap().get("eventArgs").unwrap().str();
                    if p.is_none() {
                        return;
                    }
                    p.unwrap()
                };
                let field = format!("graph:{path}");
                match core.state().await.get(&field) {
                    Some(g) => {
                        core.throw_event(Event::new("graph", MODULE_NAME, Some(g.clone())))
                            .await;
                    }
                    None => {
                        let graph = get_graph(&path);
                        core.throw_event(Event::new("graph", MODULE_NAME, Some(graph.clone())))
                            .await;
                        let mods = CoreModification::default()
                            .set_state(StateInstructionBuilder::default().add(field, graph));
                        core.get_sender()
                            .await
                            .send(mods)
                            .await
                            .expect("Channel should be opened");
                    }
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MagnoliaModule {
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

        Value::Obj(mp)
    }
}

pub(crate) fn get_graph(path: &str) -> Value {
    Value::List(
        get_modules(Path::new(path))
            .into_iter()
            .map(MagnoliaModule::to_obj)
            .collect::<Vec<Value>>(),
    )
}

pub(crate) fn get_modules(path: &Path) -> Vec<MagnoliaModule> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .flat_map(|d| match d.file_type() {
            Ok(df)
                if df.is_file() && d.file_name().to_str().is_some_and(|p| p.ends_with(".mg")) =>
            {
                vec![MagnoliaModule::new(&d.path())]
            }
            Ok(df) if df.is_dir() => get_modules(&d.path()),
            _ => Vec::new(),
        })
        .collect()
}
