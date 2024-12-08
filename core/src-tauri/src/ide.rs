use crate::setup::setup;
use anyhow::{Context, Result};
use core_std_lib::{html::thtml::THtml, map::tmap::TMap, msg::tmsg::TMsg};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[tauri::command]
async fn init() -> Vec<(String, TMap)> {
    crate::handlers::init().await
}

#[tauri::command]
async fn update(tmsg: TMsg, tmodel: TMap) -> Vec<(String, TMap)> {
    crate::handlers::update(tmsg, tmodel).await
}

#[tauri::command]
async fn view(tmodel: TMap) -> Vec<(String, THtml)> {
    crate::handlers::view(tmodel).await
}

pub async fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            setup(ide_setup(app)?).expect("Setup should always succeed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init, update, view,])
        .run(tauri::generate_context!())
        .context("IDE Application should not error")?;
    Ok(())
}

fn ide_setup(app: &mut tauri::App) -> Result<(PathBuf, PathBuf, PathBuf)> {
    #[cfg(debug_assertions)]
    {
        let dev_plugin_folder = PathBuf::new()
            .join("../plugins/")
            .canonicalize()
            .context("plugins folder should exist in development")?;

        let plugin_paths: Vec<PathBuf> = dev_plugin_folder
            .read_dir()
            .context(format!("Path: {dev_plugin_folder:?} should exist"))?
            .filter_map(|p| p.ok())
            .filter(|p| p.path().is_file())
            .filter(|p| p.file_name() != ".gitignore")
            .map(|p| p.path())
            .collect();

        let plugin_folder = app
            .path()
            .app_data_dir()
            .context("Should have permissions to read app_data_dir")?
            .join("plugins");

        fs::remove_dir_all(&plugin_folder)
            .expect("Should be able to remove directory and it's content");
        fs::create_dir_all(&plugin_folder)
            .expect("Should have permissions to create plugins folder");

        for pp in plugin_paths {
            let _ = fs::remove_file(plugin_folder.join(pp.file_name().unwrap()));
            let dest = plugin_folder.join(pp.file_name().expect("Filename should be UTF-8 safe"));
            fs::copy(&pp, &dest).context(format!("Can't copy: {pp:?}, {dest:?}"))?;
        }
    }

    let app_handle = app.app_handle();
    Ok((
        app_handle.path().app_data_dir()?,
        app_handle.path().app_config_dir()?,
        app_handle.path().app_data_dir()?.join("plugins"),
    ))
}
