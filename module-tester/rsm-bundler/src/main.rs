use core_module_lib::Module;
use core_std_lib::{html::Html, state::State};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct Core;

pub static STATE: Lazy<RwLock<State>> = Lazy::new(|| RwLock::new(State::default()));
pub static UI: Lazy<RwLock<Html>> = Lazy::new(|| RwLock::new(Html::Main()));

#[async_trait::async_trait]
impl core_std_lib::core::Core for Core {
    async fn state(&self) -> core_std_lib::state::State {
        let state = STATE.read().await;
        state.clone()
    }

    async fn ui(&self) -> core_std_lib::html::Html {
        let ui = UI.read().await;
        ui.clone()
    }

    async fn throw_event(&self, event: core_std_lib::event::Event) {
        todo!()
    }

    async fn add_handler(
        &self,
        event_name: Option<String>,
        module_name: Option<String>,
        handler_name: String,
    ) {
        todo!()
    }
}

fn main() {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
}
