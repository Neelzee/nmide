//! Contains thread safe structs for access across the application.

use crate::{app::App, core::ModuleEventRegister};
use core_module_lib::{rs_module::RsModule, Module};
use core_std_lib::{core_modification::CoreModification, html::Html, state::State};
use once_cell::sync::{Lazy, OnceCell};
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::{mpsc::Sender, RwLock};

/// HashMap, mapping module names to their corresponding runtime-module, is
/// protected by a `RwLock`, so is thread safe.
pub static RUNTIME_MODULES: tokio::sync::OnceCell<RwLock<HashMap<String, RsModule>>> =
    tokio::sync::OnceCell::const_new();

/// Path to the runtime module directory, currently not used, but could be used
/// in the future to allow for post-startup additions of modules, by "watching"
/// this folder.
pub static RUNTIME_MODULE_DIR: tokio::sync::OnceCell<PathBuf> = tokio::sync::OnceCell::const_new();

/// Path to the directory "owned" by this application, is used across instances,
/// so could be used by modules for long-term application specific values, but
/// this functionality is currently not exposed to modules.
pub static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

/// Thread safe representation of the webview. Should be represantative, but
/// changes made to the webview outside of the application, i.e. by modules
/// directly adding elements with `createElement`, are not dected. Used by
/// modules for "reading" the Html-tree.
pub static NMIDE_UI: Lazy<RwLock<Html>> = Lazy::new(|| RwLock::new(Html::Main()));

/// Thread safe state of the application, can only be changed by using the
/// application itself, so will always be representative of what is actually
/// happening. A copy is accessable by modules.
pub static NMIDE_STATE: Lazy<RwLock<State>> = Lazy::new(|| RwLock::new(State::default()));

/// Thread safe hashmap, mapping module names to a Module implementation. Not
/// accessable by other modules.
pub static COMPILE_TIME_MODULES: Lazy<RwLock<HashMap<String, Box<dyn Module>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Thread safe ModuleEventRegister, a manager struct for storing the mapping
/// between modules, and events they are triggered by. Not accessable by other
/// modules
pub static MODULE_EVENT_REGISTER: Lazy<RwLock<ModuleEventRegister>> =
    Lazy::new(|| RwLock::new(ModuleEventRegister::default()));

/// Thread safe `Sender`, a struct for sending `CoreModification`s. Is only
/// directly accessed by the Core, which clones it before passing it to
/// modules.
pub static NMIDE_SENDER: tokio::sync::OnceCell<Sender<CoreModification>> =
    tokio::sync::OnceCell::const_new();

/// Thread safe AppHandle. Used because some processes need to `emit` events,
/// which is a one-way method for the backend to communicate with the frontend.
pub static NMIDE: tokio::sync::OnceCell<Box<dyn App>> = tokio::sync::OnceCell::const_new();
