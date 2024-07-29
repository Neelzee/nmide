// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use nmide_rust_ffi::html::Html;
use once_cell::sync::Lazy;
use plugload::{Nmlugin, NmluginType};
use tauri::Window;
use tauri_plugin_log::LogTarget;
use tokio::sync::Mutex;

mod plugload;

#[derive(Clone, serde::Serialize)]
struct Payload;

#[tauri::command]
fn greet(window: Window) {
    let _ = window.emit("nmide", Payload);
}

#[tauri::command]
fn test() -> Html {
    match NMLUG.try_lock() {
        Ok(nmlug) => nmlug.view().unwrap(),
        Err(err) => panic!("{err}"),
    }
}

static NMLUG: Lazy<Mutex<Nmlugin>> = Lazy::new(|| {
    Mutex::new(
        Nmlugin::new(
            "nmide-framework",
            NmluginType::Worker,
            "/home/nmf/Documents/uib/nmide/src-tauri/plugin-libs/libnmide_framework.so",
        )
        .unwrap(),
    )
});

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing");
    println!("{:?}", test());
    println!("Testing!");
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet, test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
