use crate::core::statics::COMPILE_TIME_MODULES;
use anyhow::Result;
use core_module_lib::Module;
use log::info;
use std::{collections::HashMap, path::PathBuf};
use tauri::Manager;
use modules;

pub fn ide_setup(app: &mut tauri::App) -> Result<(PathBuf, PathBuf)> {
    let app_handle = app.app_handle();
    Ok((
        app_handle.path().app_data_dir()?,
        app_handle.path().app_data_dir()?.join("modules"),
    ))
}

pub async fn setup_compile_time_modules() -> Result<()> {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    modules::module_reg::register_modules(&mut modules);

    let str_modules = modules
        .values()
        .map(|m| (*m).name().to_string())
        .collect::<Vec<String>>();
    info!("compile-time modules: {:?}", str_modules,);

    let mut m = COMPILE_TIME_MODULES.write().await;
    *m = modules;

    Ok(())
}
