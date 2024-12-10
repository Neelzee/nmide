//! The [handlers](crate::handlers) module contains the three functions that make up the zero core
//! IDE.

use crate::statics::NMLUGS;
use core_std_lib::{
    html::thtml::THtml,
    map::{rmap::RMap, tmap::TMap},
    msg::{rmsg::RMsg, tmsg::TMsg},
};
use log::info;

// TODO: Add doc-string
pub async fn init() -> Vec<(String, TMap)> {
    info!("Backend: init");
    NMLUGS
        .get()
        .expect("Plugins are already initialized at this point")
        .iter()
        .map(|p| (p.name().to_string(), p.init().into()))
        .collect()
}

// TODO: Add doc-string
pub async fn view(tmodel: TMap) -> Vec<(String, THtml)> {
    info!("Backend: view");
    let model: RMap = tmodel.into();
    NMLUGS
        .get()
        .unwrap()
        .iter()
        .map(|p| (p.name().to_string(), p.view(&model).into()))
        .collect()
}

// TODO: Add doc-string
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
                .map(|p| (p.name().to_string(), p.update(&rmsg, &fmap).into()))
                .collect()
        }
    }
}
