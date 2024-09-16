// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Result};
use log::info;
use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::html::TSHtml;
use nmide_std_lib::{map::Map, msg::Msg};
use once_cell::sync::{Lazy, OnceCell};
use std::fs;
use std::path::PathBuf;
use tauri::{Manager, Window};
use tokio::sync::RwLock;

#[tauri::command]
async fn init(models: Vec<Map>) {
    info!("Backend: init");
    let plugins = NMLUGS.get().unwrap().read();
    let new_model = plugins
        .await
        .iter()
        .filter_map(|p| p.init().ok())
        .fold(Map::new(), |acc, m| acc.merge(m));
    let mut model = MODEL.write().await;
    let new_model = models.into_iter().fold(new_model, |acc, m| acc.merge(m));
    *model = new_model.clone();
    drop(model);
}

#[tauri::command]
async fn view(model: Map) -> Vec<TSHtml> {
    info!("Backend: view");
    NMLUGS
        .get()
        .unwrap()
        .read()
        .await
        .iter()
        .filter_map(|p| p.view(model.clone()).ok())
        .map(|h| h.into())
        .collect()
}

#[tauri::command]
async fn get_state() -> Map {
    MODEL.write().await.clone()
}

#[tauri::command]
async fn update(msg: Msg, models: Vec<Map>) {
    info!("Backend: update");
    let model = models
        .into_iter()
        .fold(MODEL.read().await.clone(), |acc, m| acc.merge(m));
    NMLUGS
        .get()
        .unwrap()
        .read()
        .await
        .iter()
        .filter_map(|p| p.update(msg.clone(), model.clone()).ok())
        .fold(model.clone(), |acc, m| acc.merge(m));
}

static NMLUGS: tokio::sync::OnceCell<RwLock<Vec<Nmlugin>>> = tokio::sync::OnceCell::const_new();
static MODEL: Lazy<RwLock<Map>> = Lazy::new(|| RwLock::new(Map::new()));

static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();
static APP_CACHE_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

const NMIDE_PLUGIN_DIR: &str = "plugins";

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(setup_handler)
        .plugin(tauri_plugin_log::Builder::default().targets([]).build())
        .invoke_handler(tauri::generate_handler![init, update, view, get_state])
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
