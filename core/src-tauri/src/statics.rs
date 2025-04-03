use core_plugin_lib::Nmlugin;
use core_std_lib::{html::rhtml::RHtml, map::rmap::RMap, NmideModule};
use once_cell::sync::{Lazy, OnceCell};
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::RwLock;

pub static NMLUGS: tokio::sync::OnceCell<Vec<Nmlugin>> = tokio::sync::OnceCell::const_new();
pub static NMIDE_PLUGIN_DIR: tokio::sync::OnceCell<PathBuf> = tokio::sync::OnceCell::const_new();
pub static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

pub static NMIDE_UI: Lazy<RwLock<RHtml>> = Lazy::new(|| RwLock::new(RHtml::empty()));
pub static NMIDE_STATE: Lazy<RwLock<RMap>> = Lazy::new(|| RwLock::new(RMap::new()));
pub static NMIDE_MODULES: Lazy<RwLock<HashMap<String, NmideModule>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub static MODULE_EVENT_REGISTER: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
