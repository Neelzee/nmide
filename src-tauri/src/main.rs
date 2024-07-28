// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use tauri::Window;
use tauri_plugin_log::LogTarget;

mod html;

#[derive(Clone, serde::Serialize)]
struct Payload;

#[tauri::command]
fn greet(window: Window) {
    let _ = window.emit("nmide", Payload);
}

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
