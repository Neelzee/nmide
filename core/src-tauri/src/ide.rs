use crate::setup::setup;
use anyhow::{Context as _, Result};
use core_std_lib::core::CoreModification;
use std::fs;
use std::path::PathBuf;
use tauri::Manager as _;

/// see [init](crate::handlers::init)
#[tauri::command]
async fn init() -> CoreModification {
    crate::handlers::init().await
}
/*

/// see [view](crate::handlers::view)
#[tauri::command]
async fn update(tmsg: TMsg, tmodel: TMap) -> Vec<(String, TMap)> {
    crate::handlers::update(tmsg, tmodel).await
}

/// see [update](crate::handlers::update)
#[tauri::command]
async fn view(tmodel: TMap) -> Vec<(String, THtml)> {
    crate::handlers::view(tmodel).await
}
*/

/// Runs the Tauri application
///
/// # Panics
///
/// Will panic if:
/// - If `../plugins` does not exist
/// - [APPDATA](https://tauri.app/reference/config/) and/or $APPDATA/plugins does not exist
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            setup(ide_setup(app).expect("IDE-setup should always succeed"));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init])
        .run(tauri::generate_context!())
        .expect("IDE Application should not error");
}

/// Gets the needed paths to $APPDATA and the module directory.
///
/// # Panics
///
/// If $APPDATA does not exist
///
/// # Errors
///
/// If either $APPDATA or $APPDATA/modules cannot be canonicalized.
fn ide_setup(app: &mut tauri::App) -> Result<(PathBuf, PathBuf)> {
    /* Development setup, copies all modules from the modules folders
     * to the $APPDATA/modules folder used by the application
     */
    #[cfg(debug_assertions)]
    {
        let dev_module_folder = PathBuf::new()
            .join("../modules/")
            .canonicalize()
            .context("modules folder should exist in development")?;

        let module_paths: Vec<PathBuf> = dev_module_folder
            .read_dir()
            .context(format!("Path: {dev_module_folder:?} should exist"))?
            .filter_map(std::result::Result::ok)
            .filter(|p| p.path().is_file())
            .filter(|p| p.file_name() != ".gitignore")
            .map(|p| p.path())
            .collect();

        let module_folder = app
            .path()
            .app_data_dir()
            .context("Should have permissions to read app_data_dir")?
            .join("plugins");

        // Ignoring the result of this function, because it only fails if the plugin_directory does
        // not exist, which is the case on the first ever run
        let _ = fs::remove_dir_all(&module_folder);

        fs::create_dir_all(&module_folder)
            .expect("Should have permissions to create modules folder");

        for pp in module_paths {
            let _ = fs::remove_file(module_folder.join(pp.file_name().unwrap()));
            let dest = module_folder.join(pp.file_name().expect("Filename should be UTF-8 safe"));
            fs::copy(&pp, &dest).context(format!("Can't copy: {pp:?}, {dest:?}"))?;
        }
    }

    let app_handle = app.app_handle();
    Ok((
        app_handle.path().app_data_dir()?,
        app_handle.path().app_data_dir()?.join("modules"),
    ))
}
