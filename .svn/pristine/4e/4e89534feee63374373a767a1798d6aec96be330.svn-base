use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{StateInstructionBuilder, Value},
};

pub struct Module;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

const MODULE_NAME: &str = "ide_tabs";

const EVENT_ADD_CONTENT: &str = "add_content";

const EVENT_CHANGE_TAB: &str = "add_content";

const STATE_KEY: &str = "ide-cache.ide_tabs_tab";

#[async_trait::async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(Event::PostInit.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(EVENT_ADD_CONTENT.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(EVENT_CHANGE_TAB.to_string(), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match event {
            Event::PostInit => {
                let mods = CoreModification::default();
                let state = StateInstructionBuilder::default().add(
                    STATE_KEY,
                    Value::List(vec![Value::new_obj().obj_add("id", Value::Int(0))]),
                );
                let ui = UIInstructionBuilder::default().add_node(
                    Html::Div()
                        .add_attr(Attr::Id("0".to_string()))
                        .add_attr(Attr::Class("show".to_string()))
                        .add_attr(Attr::Class("hideable".to_string())),
                    Some("content"),
                );
                core.get_sender()
                    .await
                    .send(mods.set_state(state).set_ui(ui))
                    .await
                    .expect("Channel should be opened");
            }
            Event::Event {
                event,
                args: Some(arg),
            } if &event == EVENT_ADD_CONTENT => {
                let content = if let Some(h) = arg.html() {
                    h
                } else {
                    arg.obj()
                        .and_then(|o| o.get("eventArgs").cloned())
                        .and_then(|o| o.html())
                        .unwrap_or_else(|| Html::Div())
                };
            }
            Event::Event {
                event,
                args: Some(arg),
            } if &event == EVENT_CHANGE_TAB => todo!(),
            _ => (),
        }
    }
}
