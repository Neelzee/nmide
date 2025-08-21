use crate::{
    core::NmideCore,
    statics::{NMIDE, NMIDE_SENDER, NMIDE_STATE, NMIDE_UI},
};
use core_std_lib::{core::Core, core_modification::CoreModification};
use log::debug;
use tokio::sync::mpsc::channel;

/// Spawns the thread handling Core Modifications.
///
/// Initializes `NMIDE_SENDER`, so should only be invoked once.
///
/// # Panics
///
/// - If `NMIDE_SENDER` already has been set.
/// - If `NMIDE` has not been set.
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
                let state = st.clone();
                let inst = ui_builder.instruction();
                let mut current_ui = NMIDE_UI.write().await;
                *current_ui = ui_builder.build(ui);
                let ui = current_ui.clone();
                let app = NMIDE.get().expect("App should be initialized");
                debug!(
                    place = "backend",
                    state:serde,
                    ui:serde,
                    pre_modification:serde,
                    pre_len = pre_modification.len(),
                    post_len = modification.len(),
                    modification:serde;
                    "received modification {:?} {:?} {:?}",
                    state,
                    ui,
                    modification
                );
                app.rerender(inst).await;
            }
        }
    });
}
