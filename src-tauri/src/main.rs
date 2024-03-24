// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{cmds::get_workspace, workspace::Workspace};
use eyre::{Context, Result};
use once_cell::sync::Lazy;
use std::path::Path;
use tauri_plugin_log::LogTarget;
use tokio::sync::Mutex;

mod cmds;
mod errors;
mod osops;
#[cfg(test)]
mod test;
mod types;
mod utils;
mod workspace;

pub static WORKSPACE: Lazy<Mutex<Workspace>> = Lazy::new(|| Mutex::new(Workspace::empty()));

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
        .invoke_handler(tauri::generate_handler![greet, get_workspace])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
