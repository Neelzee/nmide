use nmide_plugin_manager::Nmlugin;
use once_cell::sync::OnceCell;
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::RwLock;

pub static NMLUGS: tokio::sync::OnceCell<Vec<Nmlugin>> = tokio::sync::OnceCell::const_new();
pub static NMIDE_PLUGIN_DIR: tokio::sync::OnceCell<PathBuf> = tokio::sync::OnceCell::const_new();
// TODO: Check if this is needed
pub static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

// TODO: Check if this is needed
pub static APP_CACHE_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

pub static PLUGINS: tokio::sync::OnceCell<HashMap<String, &Nmlugin>> =
    tokio::sync::OnceCell::const_new();
