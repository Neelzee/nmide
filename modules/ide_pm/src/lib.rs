use std::io::Write;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::path::Path;
use core_module_lib::Module;
use core_std_lib::core::{Core, CoreModification};
use core_std_lib::event::Event;
use core_std_lib::state::{StateInstructionBuilder, Value};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl Module {
        ProjectManagerModule
    }
}

struct ProjectManagerModule;

const MODULE_NAME: &'static str = "ide_pm";
const STATE_FIELD: &'static str = "ide-pm-state";

#[async_trait::async_trait]
impl Module for ProjectManagerModule {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(
            Some("nmide://exit".to_string()),
            Some("nmide".to_string()),
            MODULE_NAME.to_string()
        ).await;
        let mods = CoreModification::default().set_state(
            StateInstructionBuilder::default()
                .add(STATE_FIELD, Value::Obj(HashMap::new()))
        );
        core.get_sender().await.send(mods).await.expect("Channel should be opened");
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match event.event_name() {
            "nmide://exit" => {
                let obj = core.state().await
                    .get(STATE_FIELD)
                    .and_then(|v| v.clone().obj())
                    .unwrap_or(HashMap::new());
                let pth =
                    &obj.get("project_path")
                        .and_then(|v| v.clone().str())
                        .unwrap_or("./.nmide.json".to_string());
                let project_path = Path::new(pth);
                if !project_path.exists() {
                    let _ = File::create(project_path).unwrap();
                }
                let mut file = OpenOptions::new().append(true).open(project_path).unwrap();
                let cache = obj.get("cache")
                    .and_then(|v| v.clone().obj())
                    .unwrap_or(HashMap::new());

                writeln!(file, "{}", serde_json::to_string_pretty(&cache).unwrap()).unwrap();
            },
            _ => ()
        }
    }
}