// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Result};
use anyhow_tauri::{IntoTAResult, TAResult};
use log::info;
use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::map::value::Value;
use nmide_std_lib::payloads::EmitMsgPayload;
use nmide_std_lib::{attr::Attr, html::Html, map::Map, msg::Msg};
use once_cell::sync::{Lazy, OnceCell};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::Window;
use tauri_plugin_log::LogTarget;
use tokio::sync::{Mutex, RwLock};

#[tauri::command]
async fn init_html() -> Html {
    info!("init_html");
    let lock = NMLUGS.get().unwrap().lock().await;
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
            let nmlugin_lock = NMLUGS.get().unwrap().lock().await;
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

static NMLUGS: tokio::sync::OnceCell<Mutex<Vec<Nmlugin>>> = tokio::sync::OnceCell::const_new();

static MODEL: Lazy<Mutex<Map>> = Lazy::new(|| Mutex::new(Map::new()));
static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();
static APP_CACHE_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

const NMIDE_PLUGIN_DIR: &str = "plugins";

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .setup(setup_handler)
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

fn development_setup(app: &mut tauri::App) -> Result<()> {
    let dev_plugin_folder = PathBuf::new()
        .join("../plugins/")
        .canonicalize()
        .context("cant canonicalize path")?;
    println!("{:?}", &dev_plugin_folder);

    let plugin_paths: Vec<PathBuf> = dev_plugin_folder
        .read_dir()
        .context(format!("Can't find path: {:?}", dev_plugin_folder))?
        .filter_map(|p| p.ok())
        .filter(|p| p.path().is_file())
        .filter(|p| p.file_name() != ".gitignore")
        .map(|p| p.path())
        .collect();

    let plugin_folder = app
        .path_resolver()
        .app_data_dir()
        .context("Failed to get app_data_dir")
        .unwrap()
        .join("plugins");

    fs::remove_dir_all(&plugin_folder)?;
    fs::create_dir(&plugin_folder)?;

    for pp in plugin_paths {
        let _ = fs::remove_file(&plugin_folder.join(pp.file_name().unwrap()));
        let dest = plugin_folder.join(pp.file_name().unwrap());
        fs::copy(&pp, &dest).context(format!("Can't copy: {pp:?}, {dest:?}"))?;
    }

    Ok(())
}

fn plugin_setup() -> Result<()> {
    let nmlugings = NMLUGS.get().unwrap().try_lock()?;
    let mut og_model = MODEL.try_lock()?;
    let model = nmlugings
        .iter()
        .filter_map(|nl| nl.init().ok())
        .fold(Map::new(), |acc, m| acc.merge(m));
    *og_model = model.merge(og_model.clone());

    Ok(())
}

fn setup_handler(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_handle = app.handle();

    APP_DATA_DIR
        .set(RwLock::new(
            app_handle.path_resolver().app_data_dir().unwrap(),
        ))
        .unwrap();

    APP_CACHE_DIR
        .set(RwLock::new(
            app_handle.path_resolver().app_config_dir().unwrap(),
        ))
        .unwrap();

    if !app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join(NMIDE_PLUGIN_DIR)
        .exists()
    {
        fs::create_dir(
            app_handle
                .path_resolver()
                .app_data_dir()
                .unwrap()
                .join(NMIDE_PLUGIN_DIR),
        )
        .unwrap();
    }

    NMLUGS.set({
        Mutex::new(
            app_handle
                .path_resolver()
                .app_data_dir()
                .unwrap()
                .join(NMIDE_PLUGIN_DIR)
                .read_dir()
                .expect("couldnt read nmide-plugin dir")
                .into_iter()
                .filter_map(|dir| match dir {
                    Ok(d)
                        if d.path().is_file()
                            && d.path().extension().is_some_and(|e| {
                                e.to_string_lossy() == "so" || e.to_string_lossy() == "dll"
                            }) =>
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
    })?;

    println!("{:?}", app_handle.path_resolver().app_log_dir());
    println!("{:?}", app_handle.path_resolver().app_cache_dir());

    #[cfg(debug_assertions)]
    {
        let res = development_setup(app);
        let _ = res.unwrap();
    }

    plugin_setup().unwrap();

    Ok(())
}
