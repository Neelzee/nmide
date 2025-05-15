use core_module_lib::Module;
use core_std_lib::core::Core;
use core_std_lib::core_modification::CoreModification;
use core_std_lib::event::Event;
use core_std_lib::state::{StateInstructionBuilder, Value};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl Module {
        ProjectManagerModule
    }
}

struct ProjectManagerModule;

const MODULE_NAME: &str = "ide_cache";
const STATE_FIELD: &str = "ide-cache";
const CACHE_PATH: &str = "./.nmide.json";

#[async_trait::async_trait]
impl Module for ProjectManagerModule {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.send_modification(
            CoreModification::default()
                .set_state(StateInstructionBuilder::default().add(STATE_FIELD, Value::new_obj())),
        )
        .await;
        core.add_handler(Event::PreExit.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(Event::PostInit.to_string(), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match event {
            Event::PostInit => {
                let pth = core
                    .state()
                    .await
                    .get("project_path")
                    .and_then(|v| v.clone().str())
                    .unwrap_or(CACHE_PATH.to_string());
                let mut file = OpenOptions::new().read(true).open(&pth).unwrap();
                let mut buf = String::new();
                let res = file.read_to_string(&mut buf);
                if res.is_err() {
                    eprintln!("Error when opening file: {:?}, error: {:?}", pth, res);
                    return;
                }
                let obj = serde_json::to_value(buf);
                if obj.is_err() {
                    eprintln!("Error when serializing: {:?}, file: {:?}", obj, pth);
                    return;
                }
                let obj: Result<Value, _> = serde_json::from_value(obj.unwrap());
                if obj.is_err() {
                    eprintln!("Error when serializing: {:?}, file: {:?}", obj, pth);
                    return;
                }
                let obj = obj.unwrap();
                let mods = CoreModification::default()
                    .set_state(StateInstructionBuilder::default().add(STATE_FIELD, obj));
                core.send_modification(mods).await;
            }
            Event::PreExit => {
                let cache = core
                    .state()
                    .await
                    .get(STATE_FIELD)
                    .and_then(|v| v.obj())
                    .unwrap_or(HashMap::new());
                let pth = &cache
                    .get("project_path")
                    .and_then(|v| v.clone().str())
                    .unwrap_or(CACHE_PATH.to_string());
                let project_path = Path::new(pth);
                if !project_path.exists() {
                    let _ = File::create(project_path).unwrap();
                }
                let mut file = OpenOptions::new().write(true).open(project_path).unwrap();
                writeln!(file, "{}", serde_json::to_string_pretty(&cache).unwrap()).unwrap();
            }
            _ => (),
        }
    }
}
