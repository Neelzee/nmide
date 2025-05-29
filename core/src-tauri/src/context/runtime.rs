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

pub struct RuntimeCore;

impl RCore for RuntimeCore {
    extern "C" fn state(&self) -> FfiFuture<RState> {
        async move {
            let state = NMIDE_STATE.read().await.clone();
            RState::from(state)
        }
        .into_ffi()
    }

    extern "C" fn ui(&self) -> FfiFuture<RHtml> {
        async move {
            let ui = NMIDE_UI.read().await.clone();
            RHtml::from(ui)
        }
        .into_ffi()
    }

    extern "C" fn throw_event(&self, event: REvent) -> FfiFuture<()> {
        NmideCore.throw_event(event.to_event()).into_ffi()
    }

    extern "C" fn add_handler(&self, event_name: RString, handler_name: RString) -> FfiFuture<()> {
        NmideCore
            .add_handler(
                event_name.as_str().to_string(),
                handler_name.as_str().to_string(),
            )
            .into_ffi()
    }

    extern "C" fn send_modification(&self, modification: RCoreModification) -> FfiFuture<()> {
        async move {
            NMIDE_SENDER
                .get()
                .expect("Should be initialized at this point")
                .send(modification.to_mod())
                .await
                .expect("Channel should be opened")
        }
        .into_ffi()
    }
}
