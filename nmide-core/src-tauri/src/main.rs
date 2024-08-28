// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use anyhow_tauri::{IntoTAResult, TAResult};
use log::info;
use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::map::value::Value;
use nmide_std_lib::{attr::Attr, html::Html, map::Map, msg::Msg};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::Window;
use tauri_plugin_log::LogTarget;
use tokio::sync::Mutex;
use ts_rs::TS;

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

#[derive(Clone, Serialize, TS)]
#[ts(export, export_to = "../../src/bindings/EmitMsgPayload.ts")]
struct EmitMsgPayload(Msg);

#[tauri::command]
async fn process_msg(window: Window, msg: Msg) -> TAResult<()> {
    info!("process_msg: `{msg:?}`");
    match msg {
        Msg::Alert(_, _) => unimplemented!(),
        Msg::OpenFolderDialog(reply_msg, _) => window
            .emit(
                "emit_msg",
                EmitMsgPayload(Msg::PluginMsg(
                    reply_msg,
                    Value::String(
                        FileDialogBuilder::new()
                            .pick_folder()
                            .and_then(|s| Some(s.to_string_lossy().to_string()))
                            .unwrap_or_default(),
                    ),
                )),
            )
            .into_ta_result(),

        _ => {
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
    }
}

#[derive(Clone, Serialize)]
struct EmptyObj;

static NMLUGS: Lazy<Mutex<Vec<Nmlugin>>> = Lazy::new(|| {
    Mutex::new(
        PathBuf::from("./plugin-libs")
            .canonicalize()
            .expect("couldnt canonicalize path")
            .read_dir()
            .expect("couldnt read nmide-plugin dir")
            .into_iter()
            .filter_map(|dir| match dir {
                Ok(d)
                    if d.path().is_file()
                        && d.path()
                            .extension()
                            .is_some_and(|e| e.to_string_lossy() == "so") =>
                {
                    Some(d.path())
                }
                Err(err) => {
                    eprintln!("Failed to get plugin path: `{err:?}`");
                    None
                }
                _ => None,
            })
            .map(|pth| {
                Nmlugin::new(&pth).expect(&format!("couldnt create plugin on path: {pth:?}"))
            })
            .collect(),
    )
});

static MODEL: Lazy<Mutex<Map>> = Lazy::new(|| Mutex::new(Map::new()));

#[tokio::main]
async fn main() -> Result<()> {
    let nmlugings = NMLUGS.try_lock()?;
    let mut og_model = MODEL.try_lock()?;
    let model = nmlugings
        .iter()
        .filter_map(|nl| nl.init().ok())
        .fold(Map::new(), |acc, m| acc.merge(m));
    *og_model = model.merge(og_model.clone());
    drop(nmlugings);
    drop(og_model);
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![init_html, process_msg,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
