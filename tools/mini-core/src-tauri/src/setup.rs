//! The [setup](crate::setup) module, for initializing the statics from [static](crate::static),
//! used by [ide](crate::ide) and [server](crate::server).

use crate::statics::APP_DATA_DIR;
use crate::statics::{RUNTIME_MODULES, RUNTIME_MODULE_DIR};
use core_module_lib::rs_module::RsModule;
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
    let (app_data, nmide_plugin) = paths;

    APP_DATA_DIR
        .set(RwLock::new(app_data))
        .expect("Initialization of APP_DATA_DIR should always succeed");

    RUNTIME_MODULE_DIR
        .set(nmide_plugin)
        .expect("Initialization of NMIDE_PLUGIN_DIR should always succeed");

    let nmide_plugin_dir = RUNTIME_MODULE_DIR.get().unwrap();
    if !nmide_plugin_dir.exists() {
        fs::create_dir_all(nmide_plugin_dir)
            .unwrap_or_else(|err| {
                panic!("Creation of the plugin directory: `{nmide_plugin_dir:?}` should succeed, failed with error: {err:?}")
            });
    }

    RUNTIME_MODULES.set(RwLock::new(
        nmide_plugin_dir
            .read_dir()
            .unwrap_or_else(|err| {
                panic!("Reading the plugin directory: `{nmide_plugin_dir:?}` should succeed, failed with error: {err:?}")
            })
            .filter_map(|dir| match dir {
                Ok(d)
                if d.path().is_file()
                && d.path().extension().is_some_and(|e| {
                // TODO: This will not work, need a cfg for os
                    e.to_string_lossy() == "so" || e.to_string_lossy() == "dll"
                }) =>
                {
                    println!("{:?}", d.path());
                    Some(d.path())
                }
                Err(err) => {
                    eprintln!("Failed to get plugin path: `{err:?}`");
                    None
                }
                _ => None,
            })
            .map(|pth| {
                // TODO: This should print to stderr, and not panic, but is useful for
                // development
                RsModule::new(pth.as_path()).unwrap_or_else(|err| {
                    panic!("Couldnt create plugin on path: {pth:?}, due too {err:?}")
                })
            })
            .map(|m| (m.name(), m))
            .collect()
    )).expect("Reading from the plugin directory should not fail");
}
