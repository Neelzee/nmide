use anyhow::{Result, anyhow};
use core_std_lib::{
    core::Core,
    core_modification::CoreModification,
    event::Event,
    state::{StateInstructionBuilder, Value},
};
use editor_ar::Art;

pub struct Module;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

const MODULE_NAME: &str = "editor_buffer";

const OPEN_BUFFER_EVENT: &str = "OPEN-BUFFER";

const BUFFER_STATE_FIELD: &str = "BUFFER";

#[async_trait::async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(OPEN_BUFFER_EVENT.to_string(), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        let res: Result<()> = match &event {
            Event::Event { event, args } if event == OPEN_BUFFER_EVENT => {
                open_buffer(args.clone(), &core).await
            }
            _ => Ok(()),
        };

        if res.is_ok() {
            return ();
        }

        let obj = Value::new_obj()
            .obj_add("error_event", Value::Str(event.event_name().to_string()))
            .obj_add("error_args", event.args().cloned().unwrap_or_default())
            .obj_add("error_msg", Value::Str(format!("{:?}", res.unwrap_err())));

        core.throw_event(Event::new("error", Some(obj))).await;
    }
}

async fn open_buffer(args: Option<Value>, core: &Box<dyn Core>) -> Result<()> {
    let buff = parse_args(args)?;
    core.send_modification(CoreModification::default().add_state(
        StateInstructionBuilder::default().add(BUFFER_STATE_FIELD, Art::parse(&buff).to_value()),
    ))
    .await;
    Ok(())
}

fn parse_args(args: Option<Value>) -> Result<String> {
    args.clone()
        .and_then(|a| {
            a.obj()
                .and_then(|obj| obj.get(BUFFER_STATE_FIELD).cloned().and_then(|a| a.str()))
                .or_else(|| a.str())
        })
        .ok_or(anyhow!("Invalid argument: {:?}", args))
}
