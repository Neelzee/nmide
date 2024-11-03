use std::fs;

use crate::statics::{NMIDE_PLUGIN_DIR, NMLUGS};
use log::info;
use nmide_std_lib::{
    html::thtml::THtml,
    map::{rmap::RMap, tmap::TMap},
    msg::{rmsg::RMsg, tmsg::TMsg},
};
use tauri::{Manager, Window};

#[tauri::command]
pub async fn install(window: Window) {
    info!("Backend: install");
    let webview = window
        .get_webview_window("main")
        .expect("Webviewcalled main should exist");
    webview
        .eval("window.plugins = new Map();")
        .expect("Evaluation should be successful");
    // TODO: This can be done before the webview is ready,
    // i.e. file paths can be gotten and read asynchronously
    fs::read_dir(
        NMIDE_PLUGIN_DIR
            .get()
            .expect("Plugin directory path should already be initialized"),
    )
    .expect("Should be able to read plugin directory")
    .filter_map(|p| match p {
        Ok(file)
            if file.path().is_file()
                && file
                    .path()
                    .extension()
                    .is_some_and(|ext| ext.to_string_lossy().ends_with("js")) =>
        {
            Some(file.path())
        }
        Ok(_) => None,
        Err(err) => {
            eprintln!("Could not read plugin: ${err:?}");
            None
        }
    })
    .for_each(|path| {
        webview
            .eval(&fs::read_to_string(path).expect("Should be able to read JS-Plugin"))
            .expect("Should be able to eval on webview");
    });
}

#[tauri::command]
pub async fn init() -> TMap {
    info!("Backend: init");
    let model = NMLUGS
        .get()
        .expect("Plugins are already initialized at this point")
        .iter()
        .map(|p| p.init())
        .fold(RMap::new(), |acc, m| acc.merge(m))
        .into();
    model
}

#[tauri::command]
pub async fn view(tmodel: TMap) -> Vec<THtml> {
    info!("Backend: view");
    let model: RMap = tmodel.into();
    NMLUGS
        .get()
        .unwrap()
        .iter()
        .map(|p| p.view(model.clone()))
        .map(|h| h.into())
        .collect::<Vec<THtml>>()
}

#[tauri::command]
pub async fn update(tmsg: TMsg, tmodel: TMap) -> TMap {
    info!("Backend: update");
    let fmap: RMap = tmodel.into();
    match &tmsg {
        TMsg::Msg(_, _) => {
            let rmsg: RMsg = tmsg.into();
            NMLUGS
                .get()
                .unwrap()
                .iter()
                .map(|p| p.update(rmsg.clone(), fmap.clone()))
                .fold(fmap.clone(), |acc, m| acc.merge(m))
                .into()
        }
    }
}
