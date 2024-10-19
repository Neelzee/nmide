use crate::{
    statics::{MODEL, NMLUGS},
    NMIDE_PLUGIN_DIR,
};
use anyhow_tauri::{IntoTAResult, TAResult};
use log::{debug, error, info};
use nmide_std_lib::{
    html::thtml::THtml,
    map::{rmap::RMap, tmap::TMap},
    msg::{rmsg::RMsg, tmsg::TMsg},
};
use serde::Serialize;
use tauri::{Emitter, Manager, Window};
use tauri_plugin_fs::FsExt;

#[derive(Serialize, Clone)]
struct EmptyPayload;

#[tauri::command]
pub async fn init(window: Window) {
    info!("Backend: init");
    let mut model = MODEL.write().await;
    *model = NMLUGS
        .get()
        .unwrap() // Is already initialized at this point
        .read()
        .await
        .iter()
        .map(|p| p.init())
        .fold(RMap::new(), |acc, m| acc.merge(m));
    drop(model);
    window.emit("view", EmptyPayload).unwrap();
}

#[tauri::command]
pub async fn view() -> Vec<THtml> {
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
pub async fn update(window: Window, msg: TMsg, model: Option<TMap>) {
    info!("Backend: update");
    let fmap: RMap = model.unwrap_or_default().into();
    let mut model = MODEL.write().await;
    match &msg {
        TMsg::Msg(_, _) => {
            let rmsg: RMsg = msg.into();
            *model = NMLUGS
                .get()
                .unwrap()
                .read()
                .await
                .iter()
                .map(|p| p.update(rmsg.clone(), model.clone()))
                .fold(fmap, |acc, m| acc.merge(m));
            window.emit("view", EmptyPayload).unwrap();
        }
    }
}

#[tauri::command]
pub async fn install(window: Window) -> TAResult<()> {
    info!("Backend: install");
    let webview = window
        .get_webview_window("main")
        .expect("Main window should exist");
    webview.eval("window.plugins = []").into_ta_result()?;
    std::fs::read_dir(
        webview
            .app_handle()
            .path()
            .app_data_dir()
            .into_ta_result()?
            .join(NMIDE_PLUGIN_DIR),
    )
    .into_ta_result()?
    .for_each(|dir| {
        if let Ok(dir_entry) = dir {
            if !dir_entry.path().is_file()
                || dir_entry
                    .path()
                    .file_name()
                    .is_some_and(|p| !p.to_str().unwrap_or_default().ends_with(".js"))
            {
                return;
            }
            match &webview.fs().read_to_string(dir_entry.path()) {
                Ok(js) => {
                    let err = webview.eval(js);
                    if err.is_ok() {
                        return;
                    }
                    error!(
                        "Failed to eval plugin: {:?} {:?}",
                        dir_entry.path(),
                        err.unwrap_err()
                    );
                }
                Err(err) => error!("Failed to eval plugin: {:?} {err:?}", dir_entry.path()),
            }
        }
    });

    Ok(())
}
