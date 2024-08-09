// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use nmide_rust_ffi::{html::Html, model::Model};
use once_cell::sync::Lazy;
use tauri_plugin_log::LogTarget;
use tokio::sync::Mutex;

use nmide_plugin_manager::Nmlugin;

#[tauri::command]
async fn init_html() -> Html {
    Html::Div {
        kids: NMLUGS
            .try_lock()
            .expect("Could not get lock on mutex")
            .iter()
            .filter_map(|nl| nl.view(Model {}).ok())
            .map(|v| v.1)
            .collect::<Vec<_>>(),
    }
}

static NMLUGS: Lazy<Mutex<Vec<Nmlugin>>> = Lazy::new(|| {
    Mutex::new(vec![Nmlugin::new(
        "nmide-framework",
        "./plugin-libs/framework.so",
    )
    .unwrap()])
});

#[tokio::main]
async fn main() -> Result<()> {
    let r = Html::Div {
        kids: NMLUGS
            .try_lock()
            .expect("Could not get lock on mutex")
            .iter()
            .filter_map(|nl| nl.view(Model {}).ok())
            .map(|v| v.1)
            .collect::<Vec<_>>(),
    };

    println!("{r:?}");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![init_html])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
