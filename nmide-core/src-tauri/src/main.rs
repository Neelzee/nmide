// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use anyhow_tauri::{IntoTAResult, TAResult};
use log::info;
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
    info!("init_html");
    let lock = NMLUGS.lock().await;
    let model_lock = MODEL.lock().await;
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
    info!("process_msg: `{msg:?}`");
    let mut model_lock = MODEL.lock().await;
    let nmlugin_lock = NMLUGS.lock().await;
    let model = nmlugin_lock.iter().fold(model_lock.clone(), |model, nl| {
        nl.update(msg.clone(), model.clone()).unwrap_or(model)
    });
    *model_lock = model;
    drop(nmlugin_lock);
    drop(model_lock);
    window.emit("refresh_html", EmptyObj).into_ta_result()
}

#[derive(Clone, Serialize)]
struct EmptyObj;

static NMLUGS: Lazy<Mutex<Vec<Nmlugin>>> = Lazy::new(|| {
    Mutex::new(vec![
        Nmlugin::new("./plugin-libs/libnmide_manager.so").unwrap()
    ])
});

static MODEL: Lazy<Mutex<Model>> = Lazy::new(|| Mutex::new(Model::new()));

#[tokio::main]
async fn main() -> Result<()> {
    let nmlugings = NMLUGS.try_lock()?;
    let mut og_model = MODEL.try_lock()?;
    let model = nmlugings
        .iter()
        .filter_map(|nl| {
            println!("{:?}", nl.manifest());
            let res = nl.init();
            println!("{res:?}");
            res.ok()
        })
        .fold(Model::new(), |acc, m| acc.merge(m));
    println!("Model:\n{model:?}");
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
