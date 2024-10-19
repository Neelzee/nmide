use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::map::rmap::RMap;
use once_cell::sync::{Lazy, OnceCell};
use std::path::PathBuf;
use tokio::sync::RwLock;

pub static NMLUGS: tokio::sync::OnceCell<RwLock<Vec<Nmlugin>>> = tokio::sync::OnceCell::const_new();
pub static MODEL: Lazy<RwLock<RMap>> = Lazy::new(|| RwLock::new(RMap::new()));

pub static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();
pub static APP_CACHE_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();
