use core_std_lib::{core::Core, html::Html, state::State};
use tauri::Emitter;

use crate::{
    ide::NMIDE,
    statics::{COMPILE_TIME_MODULES, NMIDE_STATE, NMIDE_UI},
};

pub struct NmideCore;

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
        let (new_state, new_ui) = modules
            .values()
            .map(|m| m.handler(&event, self))
            .reduce(|acc, cm| acc.combine(cm))
            .unwrap_or_default()
            .build(state, ui);

        let mut st = NMIDE_STATE.write().await;
        *st = new_state;
        println!("{st:?}");
        drop(st);
        let app = NMIDE
            .get()
            .expect("AppHandle should be initialized")
            .read()
            .await;
        app.emit("nmide://render", new_ui)
            .expect("AppHandle emit should always succeed");
    }
}
