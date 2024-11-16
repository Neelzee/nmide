use crate::statics::{NMLUGS, PLUGINS};
use anyhow::anyhow;
use anyhow_tauri::{IntoTAResult, TAResult};
use log::info;
use nmide_std_lib::{
    html::thtml::THtml,
    map::{rmap::RMap, tmap::TMap},
    msg::{rmsg::RMsg, tmsg::TMsg},
};

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

#[tauri::command]
pub async fn plugin_init(plugin_name: &str) -> TAResult<TMap> {
    if let Some(plugin) = PLUGINS
        .get()
        .expect("Should be initialized")
        .get(plugin_name)
    {
        return Ok(plugin.init().into());
    }

    Err(anyhow!("Could not find plugin: {plugin_name}")).into_ta_result()
}

#[tauri::command]
pub async fn plugin_view(plugin_name: &str, tmodel: TMap) -> TAResult<THtml> {
    if let Some(plugin) = PLUGINS
        .get()
        .expect("Should be initialized")
        .get(plugin_name)
    {
        return Ok(plugin.view(tmodel.into()).into());
    }

    Err(anyhow!("Could not find plugin: {plugin_name}")).into_ta_result()
}

#[tauri::command]
pub async fn plugin_update(plugin_name: &str, tmsg: TMsg, tmodel: TMap) -> TAResult<TMap> {
    if let Some(plugin) = PLUGINS
        .get()
        .expect("Should be initialized")
        .get(plugin_name)
    {
        return Ok(plugin.update(tmsg.into(), tmodel.into()).into());
    }

    Err(anyhow!("Could not find plugin: {plugin_name}")).into_ta_result()
}

#[tauri::command]
pub async fn get_plugins() -> Vec<String> {
    PLUGINS
        .get()
        .expect("Should be initialized")
        .keys()
        .map(|p| p.to_string())
        .collect()
}
