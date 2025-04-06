use core_module_lib::Module;
use core_std_lib::{html::Html, state::State};
use once_cell::sync::{Lazy, OnceCell};
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::RwLock;

use crate::core::NmideCore;

// TODO: Add runtime module support
pub static RUNTIME_MODULES: tokio::sync::OnceCell<Vec<()>> = tokio::sync::OnceCell::const_new();
pub static RUNTIME_MODULE_DIR: tokio::sync::OnceCell<PathBuf> = tokio::sync::OnceCell::const_new();
pub static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

pub static NMIDE_UI: Lazy<RwLock<Html>> = Lazy::new(|| RwLock::new(Html::Main()));
pub static NMIDE_STATE: Lazy<RwLock<State>> = Lazy::new(|| RwLock::new(State::default()));
pub static COMPILE_TIME_MODULES: Lazy<RwLock<HashMap<String, Module<NmideCore>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub static MODULE_EVENT_REGISTER: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
