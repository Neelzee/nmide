use crate::{
    core_modification_handler::spawn_core_modification_handler,
    setup::setup,
    statics::{NMIDE, NMIDE_STATE, NMIDE_UI},
};
use core_std_lib::{core_modification::CoreModification, event::Event, html::Html, state::Value};
use log::info;
use std::{collections::HashMap, path::PathBuf};
use tauri::{Emitter, Manager, RunEvent};
use tokio::sync::RwLock;

mod setup;

#[tauri::command]
async fn init(mods: Vec<CoreModification>) {
    info!(place = "backend", mods:serde; "init {:?}", mods);
    let cm = mods
        .into_iter()
        .fold(CoreModification::default(), CoreModification::append);
    crate::handlers::init(cm).await;
}

#[tauri::command]
async fn handler(event: Event, mods: Vec<CoreModification>) {
    info!(place = "backend", event:serde, mods:serde; "handler, {:?} {:?}", event, mods);
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

/// Runs the Tauri application
pub async fn run() {
    setup::setup_compile_time_modules()
        .await
        .expect("Compile time module setup should always succeed");

    let app = tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Folder {
                        file_name: Some("out".to_string()),
                        path: PathBuf::from("../logs"),
                    },
                ))
                .build(),
        )
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

    spawn_core_modification_handler();

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
