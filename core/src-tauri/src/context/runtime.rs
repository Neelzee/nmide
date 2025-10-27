use crate::{
    context::compile_time::NmideCore,
    core::statics::{NMIDE_SENDER, NMIDE_STATE, NMIDE_UI},
};
use abi_stable::std_types::RString;
use async_ffi::{FfiFuture, FutureExt};
use core_module_lib::rs_module::RCore;
use core_std_lib::core::Core;
use foreign_std_lib::{
    core::rs_core_modification::RCoreModification, event::rs_event::REvent, html::rs_html::RHtml,
    state::rs_state::RState,
};
use log::{error, info};

pub struct RuntimeCore;

impl RCore for RuntimeCore {
    extern "C" fn state(&self) -> FfiFuture<RState> {
        info!("[rt-core] state");
        async move {
            let state = NMIDE_STATE.read().await.clone();
            RState::from(state)
        }
        .into_ffi()
    }

    extern "C" fn ui(&self) -> FfiFuture<RHtml> {
        info!("[rt-core] ui");
        async move {
            let ui = NMIDE_UI.read().await.clone();
            RHtml::from(ui)
        }
        .into_ffi()
    }

    extern "C" fn throw_event(&self, event: REvent) -> FfiFuture<()> {
        let evt = event.to_event();
        info!("[rt-core] event: {evt:?}");
        NmideCore.throw_event(evt).into_ffi()
    }

    extern "C" fn add_handler(&self, event_name: RString, handler_name: RString) -> FfiFuture<()> {
        let evt_name = event_name.as_str().to_string();
        let hand_name = handler_name.as_str().to_string();
        info!("[rt-core] {hand_name} -> {evt_name}");
        NmideCore.add_handler(evt_name, hand_name).into_ffi()
    }

    extern "C" fn send_modification(&self, modification: RCoreModification) -> FfiFuture<()> {
        async move {
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| modification.to_mod()));

            let mods = match result {
                Ok(mods) => {
                    info!("[rt-core] modification: {mods:?}");
                    mods
                }
                Err(err) => {
                    error!("[rt-core] Panic occurred in modification.to_mod(), error: {err:#?}");
                    return;
                }
            };

            match NMIDE_SENDER.get() {
                Some(sender) => match sender.send(mods).await {
                    Ok(_) => {
                        info!("[rt-core] Modification sent successfully");
                    }
                    Err(err) => {
                        error!("[rt-core] Error sending modification: {err:?}");
                    }
                },
                None => {
                    error!("[rt-core] NMIDE_SENDER not initialized");
                }
            }
        }
        .into_ffi()
    }
}
