use crate::statics::COMPILE_TIME_MODULES;
use anyhow::Result;
use core_module_lib::Module;
use log::info;
use std::{collections::HashMap, path::PathBuf};
use tauri::Manager;

#[allow(unused_imports)]
pub mod module_reg {
    use core_module_lib::Module;
    use core_module_lib::ModuleBuilder;
    use core_std_lib::core::Core;
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/module_reg.rs"));
}

pub fn ide_setup(app: &mut tauri::App) -> Result<(PathBuf, PathBuf)> {
    let app_handle = app.app_handle();
    Ok((
        app_handle.path().app_data_dir()?,
        app_handle.path().app_data_dir()?.join("modules"),
    ))
}

pub async fn setup_compile_time_modules() -> Result<()> {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    module_reg::register_modules(&mut modules);

    let str_modules = modules
        .values()
        .map(|m| (*m).name().to_string())
        .collect::<Vec<String>>();
    info!("compile-time modules: {:?}", str_modules,);

    let mut m = COMPILE_TIME_MODULES.write().await;
    *m = modules;

    Ok(())
}
