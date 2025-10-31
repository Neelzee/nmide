use std::path::PathBuf;

use core_std_lib::{core_modification::CoreModification, event::Event, html::Html, state::State};
use futures::future::join_all;
use statics::{HANDLER_REGISTER, MODULES, STATE, UI};
use crate::core::statics::THROWN_EVENTS;

pub mod run;
pub(crate) mod statics;

struct InnerCore;

pub struct Core(InnerCore);

#[async_trait::async_trait]
impl core_std_lib::core::Core for InnerCore {
    async fn state(&self) -> State {
        STATE.read().await.clone()
    }

    async fn ui(&self) -> Html {
        UI.read().await.clone()
    }

    async fn throw_event(&self, event: Event) {
        let mut events = THROWN_EVENTS.write().await;
        events.push(event.clone());
        drop(events);
        let evt = event.event_name();
        let triggered_modules = HANDLER_REGISTER
            .read()
            .await
            .get(evt)
            .cloned()
            .unwrap_or_default();
        tokio::spawn({
            let modules = MODULES.read().await;
            async move {
                let mut module_futures = Vec::new();
                for m in triggered_modules {
                    let module = modules.get(&m);
                    if let Some(module) = module {
                        module_futures.push(module.handler(event.clone(), Box::new(InnerCore)));
                    }
                }

                join_all(module_futures).await;
            }
        });
    }

    async fn add_handler(&self, event: String, handler: String) {
        let mut register = HANDLER_REGISTER.write().await;
        let mut vec = register.get(&event).cloned().unwrap_or_default();
        vec.push(handler);
        register.insert(event, vec);
    }

    async fn send_modification(&self, modification: CoreModification) {
        let opt = modification.optimize();
        let (new_state, new_ui) = opt.build(STATE.read().await.clone(), UI.read().await.clone());
        *STATE.write().await = new_state;
        *UI.write().await = new_ui;
    }

    async fn appdir(&self) -> PathBuf {
        ".".into()
    }
}
