use crate::core::{ModuleEventRegister, NmideCore};
use core_module_lib::Module;
use core_std_lib::{html::Html, state::State};
use once_cell::sync::{Lazy, OnceCell};
use std::{collections::HashMap, path::PathBuf};
use std::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use core_std_lib::core::CoreModification;

// TODO: Add runtime module support
pub static RUNTIME_MODULES: tokio::sync::OnceCell<Vec<()>> = tokio::sync::OnceCell::const_new();
pub static RUNTIME_MODULE_DIR: tokio::sync::OnceCell<PathBuf> = tokio::sync::OnceCell::const_new();
pub static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

pub static NMIDE_UI: Lazy<RwLock<Html>> = Lazy::new(|| RwLock::new(Html::Main()));
pub static NMIDE_STATE: Lazy<RwLock<State>> = Lazy::new(|| RwLock::new(State::default()));
pub static COMPILE_TIME_MODULES: Lazy<RwLock<HashMap<String, Box<dyn Module>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub static MODULE_EVENT_REGISTER: Lazy<RwLock<ModuleEventRegister>> =
    Lazy::new(|| RwLock::new(ModuleEventRegister::default()));

pub static NMIDE_SENDER: tokio::sync::OnceCell<Sender<CoreModification>>
    = tokio::sync::OnceCell::const_new();