use async_trait::async_trait;
use core_std_lib::{
    core::Core, core_modification::CoreModification, event::Event, html::Html, state::State,
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

pub static THROWN_EVENTS: Lazy<RwLock<Vec<Event>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub struct ServerCore<'a> {
    state: State,
    ui: Html,
    handlers: &'a mut Vec<(String, String)>,
    modification: &'a mut Vec<CoreModification>,
    appdir: String,
}

#[async_trait]
impl<'a> Core for ServerCore<'a> {
    async fn state(&self) -> State {
        self.state.clone()
    }

    async fn ui(&self) -> Html {
        self.ui.clone()
    }

    async fn throw_event(&self, event: Event) {
        let mut events = THROWN_EVENTS.write().await;
        events.push(event);
    }

    async fn add_handler(&self, event: String, handler: String) {
        todo!()
    }

    async fn send_modification(
        &self,
        modification: CoreModification,
    ) {
        todo!()
    }

    async fn appdir(&self) -> std::path::PathBuf {
        todo!()
    }
}
