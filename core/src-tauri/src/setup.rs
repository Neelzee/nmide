//! The [setup](crate::setup) module, for initializing the statics from
//! [static](crate::static), used by [ide](crate::ide)

use crate::statics::APP_DATA_DIR;
use crate::statics::{RUNTIME_MODULES, RUNTIME_MODULE_DIR};
use core_module_lib::rs_module::RsModule;
use log::{info, warn};
use std::fs;
use std::path::PathBuf;
use tokio::sync::RwLock;

/// Ensures the static variables are initialized before used.
///
/// See [static](crate::static)
///
/// # Panics
///
/// Panics if $APPDATA, $APPCACHE or $APPDATA/plugins does not exist.
pub fn setup(paths: (PathBuf, PathBuf)) {
    let (app_data, nmide_module) = paths;

    APP_DATA_DIR
        .set(RwLock::new(app_data))
        .expect("Initialization of APP_DATA_DIR should always succeed");

    RUNTIME_MODULE_DIR
        .set(nmide_module)
        .expect("Initialization of NMIDE_PLUGIN_DIR should always succeed");

    let nmide_module_dir = RUNTIME_MODULE_DIR.get().unwrap();
    if !nmide_module_dir.exists() {
        fs::create_dir_all(nmide_module_dir)
            .unwrap_or_else(|err| {
                panic!("Creation of the module directory: `{nmide_module_dir:?}` should succeed, failed with error: {err:?}")
            });
    }

    RUNTIME_MODULES.set(RwLock::new(
        nmide_module_dir
            .read_dir()
            .unwrap_or_else(|err| {
                panic!("Reading the module directory: `{nmide_module_dir:?}` should succeed, failed with error: {err:?}")
            })
            .filter_map(|dir| match dir {
                Ok(d)
                if d.path().is_file()
                && d.path().extension().is_some_and(|e| {
                // TODO: This will not work, need a cfg for os
                    e.to_string_lossy() == "so" || e.to_string_lossy() == "dll"
                }) =>
                {
                    info!("{:?}", d.path());
                    Some(d.path())
                }
                Err(err) => {
                    warn!("Failed to get plugin path: `{err:?}`");
                    None
                }
                _ => None,
            })
            .filter_map(|pth| {
                // TODO: This should print to stderr, and not panic, but is useful for
                // development
                match RsModule::new(pth.as_path()) {
                    Ok(rm) => Some(rm),
                    Err(err) => {
                    warn!("Could not create module on path: {pth:?}, due too {err:?}");
                        None
                    },
                }
            })
            .map(|m| (m.name(), m))
            .collect()
    )).expect("Reading from the plugin directory should not fail");
}
