use async_trait::async_trait;
use core_module_lib::{rs_module::RsModule, Module};
use core_std_lib::{
    core::Core, core_modification::CoreModification, event::Event, html::Html, state::State,
};
use once_cell::sync::{Lazy, OnceCell};
use tokio::sync::RwLock;

pub static THROWN_EVENTS: Lazy<RwLock<Vec<Event>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub struct ServerCore<'a> {
    state: State,
    ui: Html,
    thrown_events: &'a mut Vec<Event>,
    handlers: &'a mut Vec<(String, String)>,
    modification: &'a mut Vec<CoreModification>,
    appdir: String,
}

#[async_trait]
impl<'a> Core for ServerCore<'a> {
    async fn state(&self) -> core_std_lib::state::State {
        self.state.clone()
    }

    async fn ui(&self) -> core_std_lib::html::Html {
        self.ui.clone()
    }

    async fn throw_event(&self, event: core_std_lib::event::Event) {
        self.thrown_events.push(event);
    }

    async fn add_handler(&self, event: String, handler: String) {
        todo!()
    }

    async fn send_modification(
        &self,
        modification: core_std_lib::core_modification::CoreModification,
    ) {
        todo!()
    }

    async fn appdir(&self) -> std::path::PathBuf {
        todo!()
    }
}
