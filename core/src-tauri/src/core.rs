use core_module_lib::Module;
use std::collections::HashMap;
use tokio::sync::RwLock;

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

pub struct NmideCore;

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

#[async_trait]
impl Core for NmideCore {
    async fn state(&self) -> State {
        let st = NMIDE_STATE.read().await;
        st.clone()
    }

    async fn ui(&self) -> Html {
        let ui = NMIDE_UI.read().await;
        ui.clone()
    }

    async fn throw_event(&self, event: core_std_lib::event::Event) {
        let mods = COMPILE_TIME_MODULES.read().await;
        let module_futures = MODULE_EVENT_REGISTER
            .read()
            .await
            .get_module_names(&event)
            .await
            .into_iter()
            .flat_map(|m| mods.get(&m))
            .map(|m| m.handler(&event, self));
        let state = NmideCore.state().await;
        let ui = NmideCore.ui().await;

        let cm = futures::future::join_all(module_futures)
            .await
            .into_iter()
            .reduce(|acc, cm| acc.combine(cm))
            .unwrap_or_default();

        let (new_state, ui_builder) = cm.build_state(state);

        let mut st = NMIDE_STATE.write().await;
        *st = new_state;
        drop(st);
        let app = NMIDE
            .get()
            .expect("AppHandle should be initialized")
            .read()
            .await;
        let inst = ui_builder.instruction();
        let mut current_ui = NMIDE_UI.write().await;
        // TODO: Optimize the instruction set before building
        *current_ui = ui_builder.build(ui);
        // TODO: Do a NoOp check before needlessly re-rendering
        app.emit("nmide://render", inst)
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
}
