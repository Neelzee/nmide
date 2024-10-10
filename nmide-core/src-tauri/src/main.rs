// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Result};
use log::{debug, info};
use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::{
    html::thtml::THtml,
    map::{rmap::RMap, tmap::TMap},
    msg::tmsg::TMsg,
};
use once_cell::sync::{Lazy, OnceCell};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tauri::{Emitter, Manager, Window};
use tokio::{
    fs::{read, read_dir},
    sync::RwLock,
};

#[tauri::command]
async fn uninstall_plugins(window: Window) {
    info!("Backend: uninstall_plugins");
    let webview = window.get_webview_window("main").expect("main didnt exist");
    let _ = webview.eval("window.plugins = [];");
}

#[tauri::command]
async fn install_plugins(window: Window) {
    info!("Backend: install_plugins");
    let mut entries = read_dir(format!(
        "{}/{NMIDE_PLUGIN_DIR}",
        APP_DATA_DIR
            .get()
            .expect("Should be initialized")
            .read()
            .await
            .to_string_lossy(),
    ))
    .await
    .expect("couldnt read directory");

    let webview = window.get_webview_window("main").expect("main didnt exist");
    let _ = webview.eval("window.plugins = [];");

    while let Ok(Some(entry)) = entries.next_entry().await {
        info!("{:?}", entry.file_name());
        if !entry.file_name().to_string_lossy().ends_with(".js") {
            continue;
        }
        let path = entry.path();
        let content = read(path).await.expect("couldnt read file");
        let s = String::from_utf8(content).expect("couldnt encode file-content");
        let _ = webview.eval(&s);
    }
}

#[tauri::command]
async fn init(window: Window, tmodel: TMap) {
    info!("Backend: init");
    debug!("Backend: {tmodel:?}");
    let mut model = MODEL.write().await;
    *model = NMLUGS
        .get()
        .unwrap() // Is already initialized at this point
        .read()
        .await
        .iter()
        .filter_map(|p| p.init().ok())
        .fold(tmodel.into(), |acc, m| acc.merge(m));
    let model = model.clone();
    let _ = window.emit("update", ViewPayload(model.into()));
}

#[derive(Serialize, Clone)]
struct ViewPayload(TMap);

#[tauri::command]
async fn view(model: TMap) -> Vec<THtml> {
    info!("Backend: view");
    let rmap: RMap = model.into();
    NMLUGS
        .get()
        .unwrap()
        .read()
        .await
        .iter()
        .filter_map(|p| p.view(rmap.clone()).ok())
        .map(|h| h.into())
        .collect::<Vec<THtml>>()
}

#[tauri::command]
async fn get_state() -> TMap {
    MODEL.read().await.clone().into()
}

#[tauri::command]
async fn update(_msg: TMsg, _models: Vec<TMap>) {
    info!("Backend: update");
    unimplemented!();
}

static NMLUGS: tokio::sync::OnceCell<RwLock<Vec<Nmlugin>>> = tokio::sync::OnceCell::const_new();
static MODEL: Lazy<RwLock<RMap>> = Lazy::new(|| RwLock::new(RMap::new()));

static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();
static APP_CACHE_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

const NMIDE_PLUGIN_DIR: &str = "plugins";

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(setup_handler)
        .invoke_handler(tauri::generate_handler![
            install_plugins,
            init,
            update,
            view,
            get_state,
            uninstall_plugins,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

fn development_setup(app: &mut tauri::App) -> Result<()> {
    let dev_plugin_folder = PathBuf::new()
        .join("../plugins/")
        .canonicalize()
        .context("cant canonicalize path")?;

    let plugin_paths: Vec<PathBuf> = dev_plugin_folder
        .read_dir()
        .context(format!("Can't find path: {:?}", dev_plugin_folder))?
        .filter_map(|p| p.ok())
        .filter(|p| p.path().is_file())
        .filter(|p| p.file_name() != ".gitignore")
        .map(|p| p.path())
        .collect();

    let plugin_folder = app
        .path()
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

fn setup_handler(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_handle = app.handle();

    APP_DATA_DIR
        .set(RwLock::new(app_handle.path().app_data_dir().unwrap()))
        .unwrap();

    APP_CACHE_DIR
        .set(RwLock::new(app_handle.path().app_config_dir().unwrap()))
        .unwrap();

    if !app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join(NMIDE_PLUGIN_DIR)
        .exists()
    {
        fs::create_dir(
            app_handle
                .path()
                .app_data_dir()
                .unwrap()
                .join(NMIDE_PLUGIN_DIR),
        )
        .unwrap();
    }

    NMLUGS.set({
        RwLock::new(
            app_handle
                .path()
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

    #[cfg(debug_assertions)]
    {
        let res = development_setup(app);
        let _ = res.unwrap();
    }

    Ok(())
}
