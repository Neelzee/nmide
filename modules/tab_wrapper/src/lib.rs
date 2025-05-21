use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{State, StateInstructionBuilder, Value},
};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

const MODULE_NAME: &str = "tab_wrapper";

#[async_trait::async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler("click-file".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("changed_tab".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(format!("fsa_read_{}", MODULE_NAME), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match event.event_name() {
            "click-file" => {
                let path = if event.args().is_some_and(|v| v.str().is_some()) {
                    event.args().unwrap().str().unwrap()
                } else {
                    event
                        .args()
                        .unwrap()
                        .obj()
                        .unwrap()
                        .get("eventArgs")
                        .unwrap()
                        .str()
                        .unwrap_or_default()
                };

                let pth = path.clone();
                let file_name = path.split("/").last().unwrap_or_default();

                core.send_modification(CoreModification::default().set_state(
                    StateInstructionBuilder::default().add("tab_wrapper", Value::Str(pth)),
                ))
                .await;

                core.throw_event(Event::new(
                    "add_tab",
                    Some(Value::Str(file_name.to_string())),
                ))
                .await;
            }
            "changed_tab" => {
                let path = core
                    .state()
                    .await
                    .get("tab_wrapper")
                    .and_then(|v| v.str())
                    .unwrap_or_default();
                if path.is_empty() {
                    return;
                }
                core.send_modification(
                    CoreModification::default()
                        .set_state(StateInstructionBuilder::default().remove("tab_wrapper")),
                )
                .await;
                core.throw_event(Event::new(
                    "fsa-read",
                    Some(
                        Value::new_obj()
                            .add("file_path", Value::Str(path))
                            .add("module", Value::Str(MODULE_NAME.to_string())),
                    ),
                ))
                .await;
            }
            "fsa_read_tab_wrapper" => {
                let obj = event.args().and_then(|v| v.obj()).unwrap_or_default();
                let content = obj.get("content").and_then(|v| v.str()).unwrap();
                let fp = obj.get("file_path").and_then(|v| v.str()).unwrap();
                let html = Html::Div()
                    .add_attr(Attr::Class("editor-container".to_string()))
                    .add_attr(Attr::Id("editor-div".to_string()))
                    .adopt(
                        Html::TextArea()
                            .set_text(content)
                            .add_attr(Attr::Class(format!("code-editor {}", fp.replace("/", "_"))))
                            .add_attr(Attr::Id("editor".to_string())),
                    );
                core.throw_event(Event::new("add_content", Some(Value::Html(html))))
                    .await;
            }
            _ => (),
        }
    }
}
