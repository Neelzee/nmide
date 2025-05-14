use core_module_lib::Module;
use core_std_lib::{
    attrs::Attr, core::Core, core_modification::CoreModification, event::Event,
    html::UIInstructionBuilder,
};
use tokio::sync::RwLock;
use ui::debug_ui;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        DebugModule
    }
}

mod ui;

pub struct DebugModule;

const MODULE_NAME: &str = "event_sender";

const EVENTS: [&str; 6] = [
    "Event",
    "pre-exit",
    "post-init",
    "Core Response",
    "Dialog",
    "Dialog File",
];

static TOGGLED: RwLock<bool> = RwLock::const_new(false);

#[async_trait::async_trait]
impl Module for DebugModule {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler("toggle-debug".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("event-kind-selected".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("event-form-submit".to_string(), MODULE_NAME.to_string())
            .await;
        core.send_modification(
            CoreModification::default().set_ui(
                UIInstructionBuilder::default().add_node(debug_ui(), Option::<String>::None),
            ),
        )
        .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match event.event_name() {
            "toggle-debug" => {
                let mut t = TOGGLED.write().await;
                let mods = CoreModification::default().set_ui(if *t {
                    UIInstructionBuilder::default().rem_attr(
                        Attr::Class("show-debug".to_string()),
                        "debug-container".to_string(),
                    )
                } else {
                    UIInstructionBuilder::default().add_attr(
                        "debug-container".to_string(),
                        Attr::Class("show-debug".to_string()),
                    )
                });
                *t = !*t;
                drop(t);
                core.send_modification(mods).await;
            }
            "event-kind-selected" if event.args().is_some() => {
                let obj = event.args().unwrap().obj().unwrap();
                let event = obj
                    .get("event-kind")
                    .cloned()
                    .unwrap_or_default()
                    .str()
                    .unwrap();

                let event = if event.as_str() == "Event" {
                    "event-form".to_string()
                } else {
                    event
                };

                let ui = EVENTS
                    .into_iter()
                    .fold(UIInstructionBuilder::default(), |b, e| {
                        let e = if e == "Event" { "event-form" } else { e }.to_string();

                        let e = e.replace(" ", "-").to_ascii_lowercase();

                        if e == event.as_str() {
                            b.add_attr(e.to_string(), Attr::Class("show-form".to_string()))
                        } else {
                            b.rem_attr(Attr::Class("show-form".to_string()), e.to_string())
                        }
                    });

                core.send_modification(CoreModification::ui(ui)).await;
            }
            "event-form-submit" => todo!(),
            _ => (),
        }
    }
}
