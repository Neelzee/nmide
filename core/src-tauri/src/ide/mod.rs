use crate::{
    core::NmideCore,
    setup::setup,
    statics::{NMIDE, NMIDE_SENDER, NMIDE_STATE, NMIDE_UI},
};
use core_std_lib::{
    core::Core, core_modification::CoreModification, event::Event, html::Html, state::Value,
};
use log::{debug, info};
use std::collections::HashMap;
use tauri::{Emitter, Manager, RunEvent};
use tokio::sync::{mpsc, RwLock};

mod setup;

#[tauri::command]
async fn init(mods: Vec<CoreModification>) {
    info!("[backend] init");
    let cm = mods
        .into_iter()
        .fold(CoreModification::default(), CoreModification::append);
    crate::handlers::init(cm).await;
}

#[tauri::command]
async fn handler(event: Event, mods: Vec<CoreModification>) {
    info!("[backend] handler {:?}", event);
    crate::handlers::handler(event, mods).await
}

#[tauri::command]
async fn state() -> HashMap<String, Value> {
    let st = NMIDE_STATE.read().await;
    st.clone().inner()
}

#[tauri::command]
async fn ui() -> Html {
    let ui = NMIDE_UI.read().await;
    ui.clone()
}

pub struct NmideAppData {
    pub is_exiting: bool,
}

/// Runs the Tauri application
pub async fn run() {
    setup::setup_compile_time_modules()
        .await
        .expect("Compile time module setup should always succeed");

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            NMIDE
                .set(RwLock::new(app.handle().clone()))
                .expect("AppHandle setup should always succeed");
            setup(setup::ide_setup(app).expect("IDE-setup should always succeed"));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init, state, ui, handler])
        .build(tauri::generate_context!())
        .expect("IDE Application should build successfully");

    tokio::spawn({
        let (sender, mut recv) = mpsc::channel::<CoreModification>(100);
        NMIDE_SENDER.set(sender).expect("NMIDE_SENDER not set yet");
        async move {
            while let Some(modification) = recv.recv().await {
                debug!(
                    "[backend] pre-optimized modification size {:?}",
                    modification.len()
                );
                let modification = modification.optimize();
                info!("[backend] modification size: {:?}", modification.len());
                debug!("[backend] modification: {:?}", modification);
                let state = NmideCore.state().await;
                let ui = NmideCore.ui().await;

                let (new_state, ui_builder) = modification.build_state(state);
                let mut st = NMIDE_STATE.write().await;
                *st = new_state;
                debug!("[backend] State: {:?}", st);
                drop(st);
                let app = NMIDE
                    .get()
                    .expect("AppHandle should be initialized")
                    .read()
                    .await;
                let inst = ui_builder.instruction();
                let mut current_ui = NMIDE_UI.write().await;
                *current_ui = ui_builder.build(ui);
                app.emit("nmide://render", inst)
                    .expect("AppHandle emit should always succeed");
            }
        }
    });

    app.run(move |app_handle, event| match &event {
        RunEvent::ExitRequested { .. } => app_handle
            .get_webview_window("main")
            .expect("Webview: `main` should exist")
            .destroy()
            .expect("Webview: `main` should not exist"),
        RunEvent::WindowEvent {
            event: tauri::WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            app_handle
                .emit("nmide://event", Event::pre_exit())
                .expect("Emit should succeed");
            api.prevent_close();
        }
        _ => (),
    })
}
