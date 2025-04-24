use crate::statics::COMPILE_TIME_MODULES;
use anyhow::{Context as _, Result};
use core_module_lib::Module;
use log::info;
use std::{collections::HashMap, fs, path::PathBuf};
use tauri::Manager;

#[allow(unused_imports)]
pub(super) mod module_reg {
    use core_module_lib::Module;
    use core_module_lib::ModuleBuilder;
    use core_std_lib::core::Core;
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/module_reg.rs"));
}

// TODO: Mention that this setup is only for run-time modules
//
/// Gets the needed paths to $APPDATA and the module directory.
///
/// # Panics
///
/// If $APPDATA does not exist
///
/// # Errors
///
/// If either $APPDATA or $APPDATA/modules cannot be canonicalized.
pub(super) fn ide_setup(app: &mut tauri::App) -> Result<(PathBuf, PathBuf)> {
    /* Development setup, copies all modules from the modules folders
     * to the $APPDATA/modules folder used by the application
     */
    #[cfg(debug_assertions)]
    {
        let dev_module_folder = PathBuf::new()
            .join("../modules/")
            .canonicalize()
            .context("modules folder should exist in development")?;

        let module_paths: Vec<PathBuf> = dev_module_folder
            .read_dir()
            .context(format!("Path: {dev_module_folder:?} should exist"))?
            .filter_map(Result::ok)
            .filter(|p| p.path().is_file())
            .filter(|p| p.file_name() != ".gitignore")
            .filter(|p| p.file_name() != "Modules.toml")
            .map(|p| p.path())
            .collect();

        let module_folder = app
            .path()
            .app_data_dir()
            .context("Should have permissions to read app_data_dir")?
            .join("modules");

        // Ignoring the result of this function, because it only fails if the plugin_directory does
        // not exist, which is the case on the first ever run
        let _ = fs::remove_dir_all(&module_folder);

        fs::create_dir_all(&module_folder)
            .expect("Should have permissions to create modules folder");

        for pp in module_paths {
            let dest = module_folder.join(pp.file_name().expect("Filename should be UTF-8 safe"));
            fs::copy(&pp, &dest).context(format!("Can't copy: {pp:?}, {dest:?}"))?;
        }
    }

    let app_handle = app.app_handle();
    Ok((
        app_handle.path().app_data_dir()?,
        app_handle.path().app_data_dir()?.join("modules"),
    ))
}

pub(super) async fn setup_compile_time_modules() -> Result<()> {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    module_reg::register_modules(&mut modules);

    info!(
        "[backend] modules: {:?}",
        modules.values().map(|m| (*m).name()).collect::<Vec<&str>>()
    );

    let mut m = COMPILE_TIME_MODULES.write().await;
    *m = modules;

    Ok(())
}
