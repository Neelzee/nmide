// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use abi_stable::library::{LibraryPath, RootModule};
use anyhow::{Context, Result};
use log::{debug, info};
use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::{
    html::thtml::THtml,
    map::{rmap::RMap, tmap::TMap},
    msg::{rmsg::RMsg, tmsg::TMsg},
    NmideStandardLibrary_Ref,
};
use once_cell::sync::{Lazy, OnceCell};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tauri::{Emitter, Manager, Window};
use tokio::sync::RwLock;

#[tauri::command]
async fn init(window: Window) {
    info!("Backend: init");
    let mut model = MODEL.write().await;
    *model = NMLUGS
        .get()
        .unwrap() // Is already initialized at this point
        .read()
        .await
        .iter()
        .map(|p| {
            debug!("Plugin: {p:?}");
            let m = p.init();
            let tm: TMap = m.clone().into();
            debug!("Model: {tm:?}");
            m
        })
        .fold(RMap::new(), |acc, m| acc.merge(m));
    drop(model);
    window.emit("view", EmptyPayload).unwrap();
}

#[derive(Serialize, Clone)]
struct EmptyPayload;

#[tauri::command]
async fn view() -> Vec<THtml> {
    info!("Backend: view");
    let model = MODEL.read().await;
    NMLUGS
        .get()
        .unwrap()
        .read()
        .await
        .iter()
        .map(|p| p.view(model.clone()))
        .map(|h| h.into())
        .collect::<Vec<THtml>>()
}

#[tauri::command]
async fn update(window: Window, msg: TMsg) {
    info!("Backend: update");
    let mut model = MODEL.write().await;
    let rmsg: RMsg = msg.into();
    *model = NMLUGS
        .get()
        .unwrap()
        .read()
        .await
        .iter()
        .map(|p| p.update(rmsg.clone(), model.clone()))
        .fold(RMap::new(), |acc, m| acc.merge(m));
    window.emit("view", EmptyPayload).unwrap();
}

#[tauri::command]
async fn msg(window: Window, msg: TMsg) {
    info!("Backend: msg");
    match &msg {
        TMsg::Msg(_, _) => {
            let mut model = MODEL.write().await;
            let rmsg: RMsg = msg.into();
            *model = NMLUGS
                .get()
                .unwrap()
                .read()
                .await
                .iter()
                .map(|p| p.update(rmsg.clone(), model.clone()))
                .fold(RMap::new(), |acc, m| acc.merge(m));
            window.emit("view", EmptyPayload).unwrap();
        }
    }
}

static NMLUGS: tokio::sync::OnceCell<RwLock<Vec<Nmlugin>>> = tokio::sync::OnceCell::const_new();
static MODEL: Lazy<RwLock<RMap>> = Lazy::new(|| RwLock::new(RMap::new()));

static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();
static APP_CACHE_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

const NMIDE_PLUGIN_DIR: &str = "plugins";

#[tokio::main]
async fn main() -> Result<()> {
    let path = PathBuf::from("/home/nmf/.local/share/no.nilsmf.uib/plugins/libnmide_plugin.so");
    let pp = LibraryPath::FullPath(path.as_path());
    let plugin = NmideStandardLibrary_Ref::load_from(pp)?;
    let model = plugin.init()();
    let t = RMap::new().insert("counter", 0);
    println!(
        "{:?}",
        t.lookup("counter").map(|v| v.int().unwrap_or_default())
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(setup_handler)
        .invoke_handler(tauri::generate_handler![init, update, view, msg,])
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
        .context("Failed to get app_data_dir")?
        .join("plugins");

    fs::remove_dir_all(&plugin_folder)?;
    fs::create_dir(&plugin_folder)?;

    for pp in plugin_paths {
        let _ = fs::remove_file(plugin_folder.join(pp.file_name().unwrap()));
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
                .filter_map(|dir| match dir {
                    Ok(d)
                        if d.path().is_file()
                            && d.path().extension().is_some_and(|e| {
                                e.to_string_lossy() == "so" || e.to_string_lossy() == "dll"
                            }) =>
                    {
                        println!("{:?}", d.path());
                        Some(d.path())
                    }
                    Err(err) => {
                        eprintln!("Failed to get plugin path: `{err:?}`");
                        None
                    }
                    _ => None,
                })
                .map(|pth| {
                    Nmlugin::new(pth.as_path()).unwrap_or_else(|err| {
                        panic!("Couldnt create plugin on path: {pth:?}, due too {err:?}")
                    })
                })
                .collect(),
        )
    })?;

    #[cfg(debug_assertions)]
    {
        development_setup(app).unwrap();
    }

    Ok(())
}
