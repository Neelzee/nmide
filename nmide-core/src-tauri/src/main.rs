// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use anyhow_tauri::{IntoTAResult, TAResult};
use nmide_rust_ffi::{
    attr::Attr,
    html::Html,
    model::{Model, Msg},
};
use once_cell::sync::Lazy;
use serde::Serialize;
use tauri::Window;
use tauri_plugin_log::LogTarget;
use tokio::sync::Mutex;

use nmide_plugin_manager::Nmlugin;

#[tauri::command]
async fn init_html() -> Html {
    let lock = NMLUGS.try_lock().expect("Could not get lock on mutex");
    let model_lock = MODEL
        .try_lock()
        .expect("Could not get a lock on Mutex<Model>");
    let kids = lock
        .iter()
        .filter_map(|nl| nl.view(model_lock.clone()).ok())
        .collect::<Vec<_>>();
    Html::Div {
        kids,
        attrs: vec![Attr::Id("main".to_string())],
    }
}

#[tauri::command]
async fn process_msg(window: Window, msg: Msg) -> TAResult<()> {
    let mut model_lock = MODEL.try_lock().into_ta_result()?;
    let nmlugin_lock = NMLUGS.try_lock().into_ta_result()?;
    let model = nmlugin_lock
        .iter()
        .filter_map(|nl| nl.update(msg.clone(), model_lock.clone()).ok())
        .fold(Model::new(), |acc, m| acc.merge(m));
    *model_lock = model;
    window.emit("init_html", EmptyObj).into_ta_result()
}

#[derive(Clone, Serialize)]
struct EmptyObj;

static NMLUGS: Lazy<Mutex<Vec<Nmlugin>>> = Lazy::new(|| {
    Mutex::new(vec![Nmlugin::new(
        "nmide-framework",
        "./plugin-libs/libnmide_framework.so",
    )
    .unwrap()])
});

static MODEL: Lazy<Mutex<Model>> = Lazy::new(|| Mutex::new(Model::new()));

#[tokio::main]
async fn main() -> Result<()> {
    let nmlugings = NMLUGS.try_lock()?;
    let mut og_model = MODEL.try_lock()?;
    let model = nmlugings
        .iter()
        .filter_map(|nl| nl.init().ok())
        .fold(Model::new(), |acc, m| acc.merge(m));
    *og_model = model.merge(og_model.clone());
    drop(nmlugings);
    drop(og_model);
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![init_html, process_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
