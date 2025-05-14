use core_std_lib::{core::Core, core_modification::CoreModification};
use log::debug;
use tauri::Emitter;
use tokio::sync::mpsc::channel;

use crate::{
    core::NmideCore,
    statics::{NMIDE, NMIDE_SENDER, NMIDE_STATE, NMIDE_UI},
};

pub fn spawn_core_modification_handler() {
    tokio::spawn({
        let (sender, mut recv) = channel::<CoreModification>(100);
        NMIDE_SENDER.set(sender).expect("NMIDE_SENDER not set yet");
        async move {
            while let Some(pre_modification) = recv.recv().await {
                let modification = pre_modification.clone().optimize();
                let state = NmideCore.state().await;
                let ui = NmideCore.ui().await;

                let (new_state, ui_builder) = modification.clone().build_state(state);
                let mut st = NMIDE_STATE.write().await;
                *st = new_state;
                let app = NMIDE
                    .get()
                    .expect("AppHandle should be initialized")
                    .read()
                    .await;
                let state = st.clone();
                let inst = ui_builder.instruction();
                let mut current_ui = NMIDE_UI.write().await;
                *current_ui = ui_builder.build(ui);
                let ui = current_ui.clone();
                debug!(
                    place = "backend",
                    state:serde,
                    ui:serde,
                    pre_modification:serde,
                    pre_len = pre_modification.len(),
                    post_len = modification.len(),
                    modification:serde;
                    "recieved modification {:?} {:?} {:?}",
                    state,
                    ui,
                    modification
                );
                app.emit("nmide://render", inst)
                    .expect("AppHandle emit should always succeed");
            }
        }
    });
}
