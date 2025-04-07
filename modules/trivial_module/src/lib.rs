use async_trait::async_trait;
use core_std_lib::{
    attrs::Attr,
    core::{Core, CoreModification},
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

    async fn init(&self, _core: &dyn Core) -> CoreModification {
        let state = StateInstructionBuilder::default().add("counter".to_string(), Value::Int(0));
        let ui = UIInstructionBuilder::default().add_node(
            Html::Div()
                .adopt(
                    Html::Button()
                        .text("Click")
                        .add_attr(Attr::OnClick(Event::new(
                            "counter".to_string(),
                            "trivial_module".to_string(),
                            Some(Value::Int(1)),
                        )))
                        .add_attr(Attr::Id("ButtonID".to_string())),
                )
                .adopt(
                    Html::P()
                        .text("Count: 0")
                        .add_attr(Attr::Id("PID".to_string())),
                )
                .add_attr(Attr::Id("DivId".to_string())),
            None,
            None,
        );
        CoreModification::default().set_ui(ui).set_state(state)
    }

    async fn handler(&self, event: &Event, core: &dyn Core) -> CoreModification {
        match (event.event_name(), event.args()) {
            ("counter", Some(v)) => {
                let state = StateInstructionBuilder::default().modify(
                    "counter".to_string(),
                    v.clone(),
                    |l, r| match (l, r) {
                        (Value::Int(i), Value::Int(j)) => Value::Int(i + j),
                        _ => Value::Int(0),
                    },
                );
                let ui = if let Some(Value::Int(old)) = core.state().await.get("counter") {
                    UIInstructionBuilder::default().set_text(
                        Some("PID".to_string()),
                        None,
                        format!("Count: {}", old),
                    )
                } else {
                    UIInstructionBuilder::default()
                };
                CoreModification::default().set_state(state).set_ui(ui)
            }
            _ => CoreModification::default(),
        }
    }
}
