// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use nmide_rust_ffi::{attr::Attr, html::Html, model::Model};
use once_cell::sync::Lazy;
use tauri_plugin_log::LogTarget;
use tokio::sync::Mutex;

use nmide_plugin_manager::Nmlugin;

#[tauri::command]
async fn init_html() -> Html {
    let lock = NMLUGS.try_lock().expect("Could not get lock on mutex");
    let kids = lock
        .iter()
        .filter_map(|nl| nl.view(Model::new()).ok())
        .collect::<Vec<_>>();
    Html::Div {
        kids,
        attrs: vec![Attr::Id("main".to_string())],
    }
}

static NMLUGS: Lazy<Mutex<Vec<Nmlugin>>> = Lazy::new(|| {
    Mutex::new(vec![Nmlugin::new(
        "nmide-framework",
        "./plugin-libs/libnmide_framework.so",
    )
    .unwrap()])
});

#[tokio::main]
async fn main() -> Result<()> {
    let l = NMLUGS.try_lock()?;
    drop(l);
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
