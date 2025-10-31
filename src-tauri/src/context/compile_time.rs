//! NmideCore
//!
//! A module can do the following things:
//!
//! 1. Modify the application state
//! 2. Modify the application UI
//! 3. Invoke modules
//!
//! All of which is done through the `Core`. In the following diagram, we can
//! see an example module, and how it modifies the application in its
//! _init_-state.
//!
//!```text
//!         ┌─────┐
//!         │     ▼A
//! ┌────────┐  B┌──────┐     ┌─Application─┐
//! │ Module │──►│ Core │────►│             │
//! └────────┘   └──────┘     └─────────────┘
//!         │    C▲
//!         └─────┘
//!```
//!
//! A, B, and C are different actions the module can take, labelled to make it
//! easier to visualize. The order of the actions does not matter.
//!
//! A. The module registers for an Event
//! B. The module sends a modification
//! C. The module throws an Event
//!
//! All the different actions pass through the `Core`.
//!
use crate::core::statics::{
    APP_DATA_DIR, MODULE_EVENT_REGISTER, NMIDE, NMIDE_SENDER, NMIDE_STATE, NMIDE_UI,
};
use async_trait::async_trait;
use core_std_lib::{
    core::Core, core_modification::CoreModification, event::Event, html::Html, state::State,
};
use log::info;
use std::path::PathBuf;

/// Compile-time core
///
/// Implements the `Core` trait, for use by compile-time modules.
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
        NMIDE
            .get()
            .expect("AppHandle should be initialized")
            .event(event)
            .await;
    }

    async fn add_handler(&self, event_name: String, handler_name: String) {
        let mut reg = MODULE_EVENT_REGISTER.write().await;
        reg.register_module(event_name, handler_name).await;
    }

    async fn send_modification(&self, modification: CoreModification) {
        NMIDE_SENDER
            .get()
            .expect("Sender should be initialized")
            .send(modification)
            .await
            .expect("Channel should be opened");
    }

    async fn appdir(&self) -> PathBuf {
        APP_DATA_DIR
            .get()
            .expect("Should be initialized")
            .read()
            .await
            .as_path()
            .to_path_buf()
    }
}
