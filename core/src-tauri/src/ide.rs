use crate::core::NmideCore;
use crate::setup::setup;
use crate::statics::{COMPILE_TIME_MODULES, NMIDE_STATE, NMIDE_UI};
use anyhow::{Context as _, Result};
use core_module_lib::Module;
use core_std_lib::attrs::Attr;
use core_std_lib::core::{Core, CoreModification};
use core_std_lib::event::Event;
use core_std_lib::html::Html;
use core_std_lib::instruction::Instruction;
use core_std_lib::state::Value;
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::RunEvent;
use tauri::Manager as _;
use tokio::sync::RwLock;

pub(crate) mod module_reg {
    use core_module_lib::Module;
    use core_module_lib::ModuleBuilder;
    use core_std_lib::core::Core;
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/module_reg.rs"));
}

pub static NMIDE: tokio::sync::OnceCell<RwLock<AppHandle>> = tokio::sync::OnceCell::const_new();

/// see [init](crate::handlers::init)
#[tauri::command]
async fn init(
    mods: Vec<CoreModification>,
) -> (Instruction<Html>, Instruction<String>, Instruction<Attr>) {
    info!("[backend] init");
    let cm = mods
        .into_iter()
        .fold(CoreModification::default(), CoreModification::append);
    crate::handlers::init(cm).await
}

// TODO: Implement
#[tauri::command]
async fn handler(event: Event, mods: Vec<CoreModification>) -> () {
    info!("[backend] handler {:?}", event);
    crate::handlers::handler(event, mods).await
}

#[tauri::command]
async fn state() -> HashMap<String, Value> {
    let st = NMIDE_STATE.read().await;
    st.clone().inner()
}

#[tauri::command]
async fn ui() -> Html {
    let ui = NMIDE_UI.read().await;
    ui.clone()
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

pub struct NmideAppData {
    pub is_exiting: bool,
}

/// Runs the Tauri application
///
/// # Panics
///
/// Will panic if:
/// - If `../plugins` does not exist
/// - [APPDATA](https://tauri.app/reference/config/) and/or $APPDATA/plugins does not exist
pub async fn run() {
    setup_compile_time_modules()
        .await
        .expect("Compile time module setup should always succeed");
    let mut app = tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            NMIDE
                .set(RwLock::new(app.handle().clone()))
                .expect("AppHandle setup should always succeed");
            setup(ide_setup(app).expect("IDE-setup should always succeed"));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init, state, ui, handler])
        .build(tauri::generate_context!())
        .expect("IDE Application should build successfully");

    app.run(move |app_handle, event| {
        match &event {
            RunEvent::ExitRequested { api, code, .. } => {
                app_handle
                    .get_webview_window("main")
                    .expect("Webview: `main` should exist")
                    .destroy()
                    .expect("Webview: `main` should not exist")
            }
            RunEvent::WindowEvent {
                event: tauri::WindowEvent::CloseRequested { api, .. },
                ..
            } => {
                app_handle.emit(
                    "nmide://event",
                    Event::new(
                        "nmide://exit",
                        "nmide",
                        None,
                    )
                ).expect("Emit should succeed");
                api.prevent_close();
            }
            _ => (),
        }
    })
}
// TODO: Mention that this setup is only for run-time modules
//
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
            .filter_map(Result::ok)
            .filter(|p| p.path().is_file())
            .filter(|p| p.file_name() != ".gitignore")
            .filter(|p| p.file_name() != "Modules.toml")
            .map(|p| p.path())
            .collect();

        let module_folder = app
            .path()
            .app_data_dir()
            .context("Should have permissions to read app_data_dir")?
            .join("modules");

        // Ignoring the result of this function, because it only fails if the plugin_directory does
        // not exist, which is the case on the first ever run
        let _ = fs::remove_dir_all(&module_folder);

        fs::create_dir_all(&module_folder)
            .expect("Should have permissions to create modules folder");

        for pp in module_paths {
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

async fn setup_compile_time_modules() -> Result<()> {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    module_reg::register_modules(&mut modules);

    info!(
        "[backend] modules: {:?}",
        modules
            .iter()
            .map(|(_, m)| (*m).name())
            .collect::<Vec<&str>>()
    );

    let mut m = COMPILE_TIME_MODULES.write().await;
    *m = modules;

    Ok(())
}
