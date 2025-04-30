use core_module_lib::Module;
use core_std_lib::attrs::Attr;
use core_std_lib::core::Core;
use core_std_lib::core_modification::CoreModification;
use core_std_lib::event::Event;
use core_std_lib::html::{Html, UIInstructionBuilder};
use core_std_lib::state::{StateInstructionBuilder, Value};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl Module {
        ProjectManagerModule
    }
}

struct ProjectManagerModule;

const MODULE_NAME: &'static str = "ide_pm";

fn button(text: &str) -> Html {
    let id = format!("ide-pm-{}", text.replace(" ", "-"));
    Html::Button()
        .set_text(text)
        .add_attr(Attr::Id(id.to_lowercase()))
}

fn drop_down_btn(text: &str) -> Html {
    let id = format!("ide-pm-drop-{}", text.replace(" ", "-"));
    Html::Button()
        .set_text(text)
        .add_attr(Attr::Click(Event::new(
            "ide-pm-dropdown".to_string(),
            MODULE_NAME.to_string(),
            Some(Value::Str(id.clone().to_lowercase())),
        )))
        .add_attr(Attr::Id(id.to_lowercase()))
        .add_attr(Attr::Class("dropbtn".to_string()))
}

fn navbar() -> Vec<Html> {
    vec![
        drop_down_btn("File"),
        Html::Div()
            .add_attr(Attr::Id("ide-pm-drop-file-content".to_string()))
            .add_attr(Attr::Class("dropdown-content".to_string()))
            .adopt(button("New File"))
            .adopt(button("Open File").add_attr(Attr::Click(Event::new(
                "ide-pm-file".to_string(),
                MODULE_NAME.to_string(),
                None,
            )))),
        Html::Button().set_text("Edit"),
        Html::Button().set_text("Selection"),
        Html::Button().set_text("View"),
        Html::Button().set_text("Go"),
        Html::Button().set_text("Run"),
        Html::Button().set_text("Terminal"),
        Html::Button().set_text("Help"),
    ]
}

#[async_trait::async_trait]
impl Module for ProjectManagerModule {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(Some("post-init".to_string()), None, MODULE_NAME.to_string())
            .await;
        core.add_handler(
            Some("ide-pm-dropdown".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        core.add_handler(
            Some("ide-pm-file".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        core.add_handler(
            Some("nmide://file".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        core.add_handler(
            Some("fsa-read-ide_pm".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        let sender = core.get_sender().await;
        match event.event_name() {
            "post-init" => {
                let mods = UIInstructionBuilder::default().add_nodes(navbar(), Some("navbar"));
                sender
                    .send(CoreModification::ui(mods))
                    .await
                    .expect("Channel should be open");
            }
            "ide-pm-dropdown" if event.args().is_some() => {
                let state = core.state().await;
                let id = match event.args().unwrap() {
                    Value::Str(s) => s.to_string(),
                    Value::Obj(obj) => obj
                        .clone()
                        .to_hm()
                        .get("eventArgs")
                        .cloned()
                        .and_then(|v| v.str())
                        .unwrap(),
                    _ => panic!("Unhallowed argument"),
                };
                let id = format!("{id}-content");
                let toggle = !state.get(&id).and_then(|v| v.bool()).is_some_and(|v| v);
                let mods = if toggle {
                    UIInstructionBuilder::default()
                        .add_attr(id.clone(), Attr::Class("show".to_string()))
                } else {
                    UIInstructionBuilder::default()
                        .rem_attr(Attr::Class("show".to_string()), id.clone())
                };
                sender
                    .send(
                        CoreModification::ui(mods).set_state(
                            StateInstructionBuilder::default().set(id, Value::Bool(toggle)),
                        ),
                    )
                    .await
                    .expect("Channel should be open");
            }
            "ide-pm-file" => {
                core.throw_event(Event::new("nmide://file?", MODULE_NAME, None))
                    .await;
            }
            "nmide://file" => {
                core.throw_event(Event::new("fsa-read", MODULE_NAME, event.args().cloned()))
                    .await;
            }
            "fsa-read-ide_pm" => {
                println!("DATA: {:?}", event.args());
            }
            _ => (),
        }
    }
}
