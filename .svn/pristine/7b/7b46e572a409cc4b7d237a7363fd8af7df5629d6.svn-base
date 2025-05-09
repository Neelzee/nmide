use crate::core_modification::CoreModification;
use crate::{event::Event, html::Html, state::State};
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait Core: Send + Sync {
    async fn state(&self) -> State;
    async fn ui(&self) -> Html;
    async fn throw_event(&self, event: Event);
    async fn add_handler(&self, event_name: String, handler_name: String);
    async fn get_sender(&self) -> Sender<CoreModification>;
}
