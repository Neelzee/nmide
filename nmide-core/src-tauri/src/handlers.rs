// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::statics::{MODEL, NMLUGS};
use log::{debug, info};
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
pub async fn update(window: Window, msg: TMsg) {
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
pub async fn msg(window: Window, msg: TMsg) {
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
