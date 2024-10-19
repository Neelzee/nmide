// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::statics::{MODEL, NMLUGS};
use log::info;
use nmide_std_lib::{
    html::thtml::THtml,
    map::{rmap::RMap, tmap::TMap},
    msg::{rmsg::RMsg, tmsg::TMsg},
};
use serde::Serialize;
use tauri::{Emitter, Window};

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
