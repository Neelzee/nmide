// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::ide::cmds::{get_content, get_workspace};
use crate::nmide::WORKSPACE;
use eyre::{Context, Result};
use tauri_plugin_log::LogTarget;

mod ide;
mod nmide;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tokio::main]
async fn main() -> Result<()> {
    let g = WORKSPACE.try_lock().wrap_err("Failed")?;

    drop(g);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet, get_workspace, get_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
