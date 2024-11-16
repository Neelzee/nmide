// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::handlers::{get_plugins, init, plugin_init, plugin_update, plugin_view, update, view};
use crate::setup::{development_setup, setup};
use anyhow::Result;

mod handlers;
mod setup;
mod statics;

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            setup(app).expect("Setup should always succeed");
            #[cfg(debug_assertions)]
            {
                development_setup(app)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init,
            update,
            view,
            plugin_init,
            plugin_update,
            plugin_view,
            get_plugins,
        ])
        .run(tauri::generate_context!())
        .expect("Application should not error");
    Ok(())
}
