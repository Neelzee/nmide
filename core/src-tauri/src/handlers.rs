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

#[tauri::command]
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
                .map(|p| (p.name().to_string(), p.update(&rmsg, &fmap).into()))
                .collect::<Vec<(String, TMap)>>()
        }
    }
}

#[tauri::command]
pub async fn plugin_init(plugin_name: String) -> TAResult<TMap> {
    info!("Backend: plugin_init-{plugin_name}");
    if let Some(plugin) = PLUGINS
        .get()
        .expect("Should be initialized")
        .get(&plugin_name)
    {
        return Ok(plugin.init().into());
    }

    Err(anyhow!("Could not find plugin: {plugin_name}")).into_ta_result()
}

#[tauri::command]
pub async fn plugin_view(plugin_name: String, tmodel: TMap) -> TAResult<THtml> {
    info!("Backend: plugin_view-{plugin_name}");
    if let Some(plugin) = PLUGINS
        .get()
        .expect("Should be initialized")
        .get(&plugin_name)
    {
        return Ok(plugin.view(&tmodel.into()).into());
    }

    Err(anyhow!("Could not find plugin: {plugin_name}")).into_ta_result()
}

#[tauri::command]
pub async fn plugin_update(plugin_name: String, tmsg: TMsg, tmodel: TMap) -> TAResult<TMap> {
    info!("Backend: plugin_update-{plugin_name}");
    if let Some(plugin) = PLUGINS
        .get()
        .expect("Should be initialized")
        .get(&plugin_name)
    {
        return Ok(plugin.update(&tmsg.into(), &tmodel.into()).into());
    }

    Err(anyhow!("Could not find plugin: {plugin_name}")).into_ta_result()
}

#[tauri::command]
pub async fn get_plugins() -> Vec<String> {
    info!("Backend: get_plugins");
    PLUGINS
        .get()
        .expect("Should be initialized")
        .keys()
        .map(|p| p.to_string())
        .collect()
}
