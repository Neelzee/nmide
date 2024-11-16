use crate::statics::{NMIDE_PLUGIN_DIR, NMLUGS, PLUGINS};
use anyhow::{Context, Result};
use nmide_plugin_manager::Nmlugin;
use std::path::PathBuf;
use std::{collections::HashMap, fs};
use tauri::Manager;
use tokio::sync::RwLock;

use crate::statics::{APP_CACHE_DIR, APP_DATA_DIR};

pub fn setup(app: &mut tauri::App) -> Result<()> {
    let app_handle = app.handle();

    APP_DATA_DIR
        .set(RwLock::new(app_handle.path().app_data_dir().unwrap()))
        .expect("Initialization of NMIDE_PLUGIN_DIR should always succeed");

    APP_CACHE_DIR
        .set(RwLock::new(app_handle.path().app_config_dir().unwrap()))
        .expect("Initialization of NMIDE_PLUGIN_DIR should always succeed");

    NMIDE_PLUGIN_DIR
        .set(app_handle.path().app_data_dir().unwrap().join("plugins"))
        .expect("Initialization of NMIDE_PLUGIN_DIR should always succeed");

    let nmide_plugin_dir = NMIDE_PLUGIN_DIR.get().unwrap();
    if !nmide_plugin_dir.exists() {
        fs::create_dir_all(nmide_plugin_dir)
            .unwrap_or_else(|err| {
                panic!("Creation of the plugin directory: `{nmide_plugin_dir:?}` should succeed, failed with error: {err:?}")
            });
    }

    NMLUGS.set({
        nmide_plugin_dir
            .read_dir()
            .unwrap_or_else(|err| {
                panic!("Reading the plugin directory: `{nmide_plugin_dir:?}` should succeed, failed with error: {err:?}")
            })
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
                // TODO: This should print to stderr, and not panic, but is useful for
                // development
                Nmlugin::new(pth.as_path()).unwrap_or_else(|err| {
                    panic!("Couldnt create plugin on path: {pth:?}, due too {err:?}")
                })
            })
            .collect()
    })?;

    PLUGINS.set({
        let plugins = NMLUGS.get().expect("Should already be Initialized");

        let mut map = HashMap::new();
        for plugin in plugins {
            map.insert(plugin.name().to_string(), plugin);
        }

        map
    })?;
    Ok(())
}

pub fn development_setup(app: &mut tauri::App) -> Result<()> {
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
    fs::create_dir_all(&plugin_folder).expect("Should have permissions to create plugins folder");

    for pp in plugin_paths {
        let _ = fs::remove_file(plugin_folder.join(pp.file_name().unwrap()));
        let dest = plugin_folder.join(pp.file_name().expect("Filename should be UTF-8 safe"));
        fs::copy(&pp, &dest).context(format!("Can't copy: {pp:?}, {dest:?}"))?;
    }

    Ok(())
}
