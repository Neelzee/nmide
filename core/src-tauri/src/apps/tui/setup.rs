use crate::core::statics::COMPILE_TIME_MODULES;
use anyhow::Result;
use core_module_lib::Module;
use log::info;
use modules;
use std::{collections::HashMap, path::PathBuf};

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
