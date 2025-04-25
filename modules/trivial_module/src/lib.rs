use async_trait::async_trait;
use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{StateInstructionBuilder, Value},
};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

#[async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        "trivial module"
    }

    async fn init(&self, core: Box<dyn Core>) {
        let state = StateInstructionBuilder::default().add("counter".to_string(), Value::Int(0));
        let ui = UIInstructionBuilder::default().add_node(
            Html::Div()
                .adopt(
                    Html::Button()
                        .set_text("Click")
                        .add_attr(Attr::Click(Event::new(
                            "counter".to_string(),
                            "trivial_module".to_string(),
                            Some(Value::Int(1)),
                        )))
                        .add_attr(Attr::Id("ButtonID".to_string())),
                )
                .adopt(
                    Html::P()
                        .set_text("Count: 0")
                        .add_attr(Attr::Id("PID".to_string())),
                )
                .add_attr(Attr::Id("DivId".to_string())),
            None,
            None,
        );
        core.add_handler(
            Some("counter".to_string()),
            None,
            "trivial_module".to_string(),
        )
        .await;
        let mods = CoreModification::default().set_ui(ui).set_state(state);
        core.get_sender()
            .await
            .send(mods)
            .await
            .expect("Channel should be opened");
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        let sender = core.get_sender().await;
        println!("{:?}", event.args());
        match (event.event_name(), event.args()) {
            ("counter", Some(v))
                if v.is_int() || v.obj().is_some_and(|o| o.contains_key("eventArgs")) =>
            {
                let state = StateInstructionBuilder::default().set(
                    "counter".to_string(),
                    if v.is_int() {
                        v.clone()
                    } else {
                        v.obj().unwrap().get("eventArgs").unwrap().clone()
                    },
                );
                let ui = if let Some(v) = core.state().await.get("counter") {
                    UIInstructionBuilder::default().set_text(
                        Some("PID".to_string()),
                        None,
                        format!(
                            "Count: {}",
                            if v.is_int() {
                                v.clone().int().unwrap()
                            } else {
                                v.clone()
                                    .obj()
                                    .and_then(|o| o.get("eventArgs").cloned())
                                    .and_then(|val| val.clone().int())
                                    .unwrap_or(0)
                            }
                        ),
                    )
                } else {
                    UIInstructionBuilder::default()
                };
                let mods = CoreModification::default().set_state(state).set_ui(ui);
                sender.send(mods).await.expect("Channel should be opened");
            }
            _ => (),
        }
    }
}
