use crate::core_modification::CoreModification;
use crate::{event::Event, html::Html, state::State};
use async_trait::async_trait;

#[async_trait]
pub trait Core: Send + Sync {
    async fn state(&self) -> State;
    async fn ui(&self) -> Html;
    async fn throw_event(&self, event: Event);
    async fn add_handler(&self, event: String, handler: String);
    async fn send_modification(&self, modification: CoreModification);
}
