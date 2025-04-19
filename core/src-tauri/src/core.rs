use core_module_lib::Module;
use tokio::sync::mpsc::{self, Receiver, Sender};
use std::collections::HashMap;
use tokio::sync::RwLock;
use log::info;

use crate::{
    ide::NMIDE,
    statics::{COMPILE_TIME_MODULES, MODULE_EVENT_REGISTER, NMIDE_STATE, NMIDE_UI},
};
use async_trait::async_trait;
use core_std_lib::{
    core::Core,
    event::Event,
    html::{Html},
    state::State,
};
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use core_std_lib::core::CoreModification;
use crate::statics::NMIDE_SENDER;

#[derive(Default)]
pub struct ModuleEventRegister {
    event: RwLock<HashMap<String, Vec<String>>>,
    module: RwLock<HashMap<String, Vec<String>>>,
}

impl ModuleEventRegister {
    pub async fn get_module_names(&self, event: &Event) -> Vec<String> {
        let mut modules = Vec::new();

        modules.append(
            &mut self
                .event
                .read()
                .await
                .get(event.event_name())
                .cloned()
                .unwrap_or(Vec::new()),
        );
        modules.append(
            &mut self
                .module
                .read()
                .await
                .get(event.module_name())
                .cloned()
                .unwrap_or(Vec::new()),
        );

        modules
    }

    pub async fn register_module(
        &mut self,
        event: Option<String>,
        module: Option<String>,
        handler: String,
    ) {
        info!(
            "[backend][handler-register] register module: {}, to event {:?}, to module name {:?}",
            handler,
            event,
            module
        );
        if let Some(evt) = event {
            let mut modules = self.event.write().await;
            let mut vec = modules.get(&evt).cloned().unwrap_or(Vec::new());
            vec.push(handler.clone());
            modules.insert(evt, vec);
        }
        if let Some(md) = module {
            let mut modules = self.event.write().await;
            let mut vec = modules.get(&md).cloned().unwrap_or(Vec::new());
            vec.push(handler.clone());
            modules.insert(md, vec);
        }
    }
}

pub struct NmideCore;

#[async_trait]
impl Core for NmideCore {
    async fn state(&self) -> State {
        info!("[backend] state");
        let st = NMIDE_STATE.read().await;
        st.clone()
    }

    async fn ui(&self) -> Html {
        info!("[backend] ui");
        let ui = NMIDE_UI.read().await;
        ui.clone()
    }

    async fn throw_event(&self, event: Event) {
        let app = NMIDE
            .get()
            .expect("AppHandle should be initialized")
            .read()
            .await;
        app.emit("nmide://event", event)
            .expect("AppHandle emit should always succeed");
    }

    async fn add_handler(
        &self,
        event_name: Option<String>,
        module_name: Option<String>,
        handler_name: String,
    ) {
        let mut reg = MODULE_EVENT_REGISTER.write().await;
        reg.register_module(event_name, module_name, handler_name)
            .await;
    }

    async fn get_sender(&self) -> Sender<CoreModification> {
        NMIDE_SENDER.get().expect("Sender should be initialized").clone()
    }
}
