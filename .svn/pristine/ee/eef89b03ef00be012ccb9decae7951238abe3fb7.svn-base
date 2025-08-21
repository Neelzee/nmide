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
        core.add_handler("counter".to_string(), "trivial_module".to_string())
            .await;
        let mods = CoreModification::default().set_state(state);
        core.send_modification(mods).await;
        let ui = Html::Div()
            .adopt(
                Html::Button()
                    .set_text("Click")
                    .add_attr(Attr::Click(Event::new(
                        "counter".to_string(),
                        Some(Value::Int(1)),
                    )))
                    .add_attr(Attr::Id("ButtonID".to_string())),
            )
            .adopt(
                Html::P()
                    .set_text("Count: 0")
                    .add_attr(Attr::Id("PID".to_string())),
            )
            .add_attr(Attr::Id("DivId".to_string()));
        core.add_handler("post-ide-pm".to_string(), "trivial_module".to_string())
            .await;
        core.throw_event(Event::new("add_content", Some(Value::Html(ui))))
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match (event.event_name(), event.args()) {
            ("post-ide-pm", _) => {
                core.throw_event(Event::new("refresh_tab", None)).await;
            }
            ("counter", Some(v))
                if v.is_int() || v.obj().is_some_and(|o| o.contains_key("eventArgs")) =>
            {
                let add = if v.is_int() {
                    v.clone().int().unwrap()
                } else {
                    v.obj()
                        .unwrap()
                        .get("eventArgs")
                        .unwrap()
                        .clone()
                        .int()
                        .unwrap()
                };
                let new_count = core
                    .state()
                    .await
                    .get("counter")
                    .unwrap_or(Value::Int(0))
                    .map(|i: i32| i + add)
                    .unwrap();
                let state = StateInstructionBuilder::default()
                    .set("counter".to_string(), new_count.clone());
                let ui = UIInstructionBuilder::default()
                    .set_text(Some("PID"), format!("Count: {}", new_count.int().unwrap()));
                let mods = CoreModification::default().set_state(state).set_ui(ui);
                core.send_modification(mods).await;
            }
            _ => (),
        }
    }
}
