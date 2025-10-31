use core_module_lib::Module;
use core_std_lib::{html::Html, state::State};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::RwLock;
use core_std_lib::event::Event;

pub static STATE: Lazy<RwLock<State>> = Lazy::new(|| RwLock::new(State::default()));
pub static UI: Lazy<RwLock<Html>> = Lazy::new(|| RwLock::new(Html::Div()));
pub static MODULES: Lazy<RwLock<HashMap<String, Box<dyn Module>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
pub static HANDLER_REGISTER: Lazy<RwLock<HashMap<String, Vec<String>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub static THROWN_EVENTS: Lazy<RwLock<Vec<Event>>> = Lazy::new(|| RwLock::new(Vec::new()));
