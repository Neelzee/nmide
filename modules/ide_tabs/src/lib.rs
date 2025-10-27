use core_std_lib::{
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::UIBuilder,
    state::{Value, state_builder::StateBuilder},
};
use post_init::build_from_storage;

mod event;
mod post_init;

pub struct Module;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub const ID_TAB_BTN_CONTAINER: &str = "tab-btn-container";

pub const ID_TAB_CONTAINER: &str = "ide-tab-container";

pub const MODULE_NAME: &str = "ide_tabs";

pub const EVENT_ADD_CONTENT: &str = "add_content";

pub const EVENT_CHANGE_TAB: &str = "change_tab";

pub const EVENT_CHANGED_TAB: &str = "changed_tab";

pub const EVENT_ADD_TAB: &str = "add_tab";

pub const EVENT_REM_TAB: &str = "rem_tab";

pub const EVENT_REFRESH_TAB: &str = "refresh_tab";

pub const STATE_TABS: &str = "ide_tabs_tabs";

pub const STATE_CURRENT_TAB_KEY: &str = "ide_tabs_current_tab";

pub const STATE_TAB_STORAGE: &str = "ide_tabs_storage";

pub const STATE_INITIALIZED: &str = "ide_tabs_init";

pub const HIDE_TAB_CLASS: &str = "hide-tab";

pub const SHOW_TAB_CLASS: &str = "show-tab";

#[async_trait::async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.send_modification(
            CoreModification::default()
                .set_state(StateBuilder::default().add(STATE_TAB_STORAGE, Value::List(Vec::new()))),
        )
        .await;
        core.add_handler("post-ide-pm".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(EVENT_ADD_CONTENT.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(EVENT_CHANGE_TAB.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(EVENT_ADD_TAB.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(EVENT_REM_TAB.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(EVENT_REFRESH_TAB.to_string(), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match &event {
            Event::Event { event: evt, .. } if evt == "post-ide-pm" => {
                post_init::handler(core).await
            }
            Event::Event { event: evt, .. } if evt == EVENT_ADD_CONTENT => {
                event::add_content_handler(event, core).await;
            }
            Event::Event { event: evt, .. } if evt == EVENT_CHANGE_TAB => {
                event::change_handler(event, core).await;
            }
            Event::Event { event: evt, .. } if evt == EVENT_ADD_TAB => {
                event::tab_add_handler(event, core).await;
            }
            Event::Event { event: evt, .. } if evt == EVENT_REFRESH_TAB => {
                let xs = core
                    .state()
                    .await
                    .get(STATE_TAB_STORAGE)
                    .and_then(|v| v.list())
                    .unwrap_or_default();
                if xs.is_empty() {
                    return;
                }
                core.send_modification(
                    CoreModification::default()
                        .set_ui(build_from_storage(xs, UIBuilder::default()))
                        .set_state(
                            StateBuilder::default()
                                .set(STATE_CURRENT_TAB_KEY, Value::List(Vec::new())),
                        ),
                )
                .await;
            }
            Event::Event { event: evt, .. } if evt == EVENT_REM_TAB => {
                todo!();
            }
            _ => (),
        }
    }
}
