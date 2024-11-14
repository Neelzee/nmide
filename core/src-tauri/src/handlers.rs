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
pub async fn update(tmsg: TMsg, tmodel: TMap) -> Vec<(String, TMap)> {
    info!("Backend: update");
    let fmap: RMap = tmodel.into();
    match &tmsg {
        TMsg::Msg(_, _) => {
            let rmsg: RMsg = tmsg.into();
            NMLUGS
                .get()
                .unwrap()
                .iter()
                .map(|p| {
                    (
                        p.name().to_string(),
                        p.update(rmsg.clone(), fmap.clone()).into(),
                    )
                })
                .collect::<Vec<(String, TMap)>>()
        }
    }
}
