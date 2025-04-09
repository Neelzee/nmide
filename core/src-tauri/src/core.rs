use crate::{
    ide::NMIDE,
    statics::{COMPILE_TIME_MODULES, NMIDE_STATE, NMIDE_UI},
};
use async_trait::async_trait;
use core_std_lib::{
    core::Core,
    html::{Html, UIInstruction},
    state::State,
};
use serde::{Deserialize, Serialize};
use tauri::Emitter;

pub struct NmideCore;

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
        let modules = COMPILE_TIME_MODULES.read().await;
        let state = NmideCore.state().await;
        let ui = NmideCore.ui().await;
        let module_futures = modules.values().map(|m| m.handler(&event, self));

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
        let inst = ui_builder.get_instructions();
        let mut current_ui = NMIDE_UI.write().await;
        *current_ui = ui_builder.build(ui);
        app.emit("nmide://render", inst)
            .expect("AppHandle emit should always succeed");
    }
}
