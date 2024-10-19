// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::handlers::{init, install, update, view};
use crate::setup::{development_setup, setup};
use anyhow::Result;

mod handlers;
mod setup;
mod statics;

const NMIDE_PLUGIN_DIR: &str = "plugins";

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            setup(app)?;
            #[cfg(debug_assertions)]
            {
                development_setup(app)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init, update, view, install])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
