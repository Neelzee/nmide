use core_plugin_lib::Nmlugin;
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tokio::sync::RwLock;

pub static NMLUGS: tokio::sync::OnceCell<Vec<Nmlugin>> = tokio::sync::OnceCell::const_new();
pub static NMIDE_PLUGIN_DIR: tokio::sync::OnceCell<PathBuf> = tokio::sync::OnceCell::const_new();
pub static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();
