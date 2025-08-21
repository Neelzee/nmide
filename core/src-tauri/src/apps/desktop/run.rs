use crate::{
    context::compile_time::NmideCore,
    core::{
        modification_handler::spawn_core_modification_handler,
        setup::setup,
        statics::{NMIDE, NMIDE_STATE, NMIDE_UI},
    },
    platform::tauri::TauriPlatform,
};
use anyhow::{Context, Result};
use core_std_lib::{
    core::Core, core_modification::CoreModification, event::Event, html::Html, state::Value,
};
use log::info;
use std::{collections::HashMap, path::PathBuf};
use tauri::{Emitter, Manager, RunEvent};

#[tauri::command]
async fn init() {
    info!("[Backend] init");
    crate::core::handlers::init().await
}

#[tauri::command]
async fn handler(event: Event) {
    info!("[Backend] handler, {:?}", event);
    crate::core::handlers::handler(event).await
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

#[tauri::command]
async fn modification(modification: CoreModification) {
    NmideCore.send_modification(modification).await;
}

/// Runs the Tauri application
pub async fn run() -> Result<usize> {
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
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            setup(super::setup::ide_setup(app).expect("IDE-setup should always succeed"));
            NMIDE
                .set(Box::new(TauriPlatform::new(app.handle().clone())))
                .unwrap_or_else(|_| panic!("AppHandle setup should always succeed"));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init,
            state,
            ui,
            handler,
            modification
        ])
        .build(tauri::generate_context!())
        .context("IDE Application should build successfully")?;

    spawn_core_modification_handler();

    let exitcode = app.run_return(move |app_handle, event| match &event {
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
    });

    Ok(exitcode as usize)
}
