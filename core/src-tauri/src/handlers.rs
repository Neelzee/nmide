use crate::statics::NMLUGS;
use log::info;
use nmide_std_lib::{
    html::thtml::THtml,
    map::{rmap::RMap, tmap::TMap},
    msg::{rmsg::RMsg, tmsg::TMsg},
};

pub async fn init() -> Vec<(String, TMap)> {
    info!("Backend: init");
    let model = NMLUGS
        .get()
        .expect("Plugins are already initialized at this point")
        .iter()
        .map(|p| (p.name().to_string(), p.init().into()))
        .collect();
    model
}

pub async fn view(tmodel: TMap) -> Vec<(String, THtml)> {
    info!("Backend: view");
    let model: RMap = tmodel.into();
    NMLUGS
        .get()
        .unwrap()
        .iter()
        .map(|p| (p.name().to_string(), p.view(&model).into()))
        .collect::<Vec<(String, THtml)>>()
}

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
                .collect::<Vec<(String, TMap)>>()
        }
    }
}
