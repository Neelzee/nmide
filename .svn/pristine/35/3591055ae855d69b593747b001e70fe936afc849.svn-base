use crate::core::{
    InnerCore,
    statics::{HANDLER_REGISTER, MODULES, STATE, UI},
};
use core_module_lib::Module;
use core_std_lib::{event::Event, html::Html, state::State};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct CoreOptions {
    starting_state: State,
    starting_ui: Html,
    events_thrown_pre_init: Vec<Event>,
    events_thrown_post_init: Vec<Event>,
    event_register: HashMap<String, Vec<String>>,
}

impl CoreOptions {
    pub fn state(self, starting_state: State) -> Self {
        Self {
            starting_state,
            ..self
        }
    }

    pub fn ui(self, starting_ui: Html) -> Self {
        Self {
            starting_ui,
            ..self
        }
    }

    pub fn pre_init_events(self, events_thrown_pre_init: Vec<Event>) -> Self {
        Self {
            events_thrown_pre_init,
            ..self
        }
    }

    pub fn post_init_events(self, events_thrown_post_init: Vec<Event>) -> Self {
        Self {
            events_thrown_post_init,
            ..self
        }
    }

    pub fn event_register(self, event_register: HashMap<String, Vec<String>>) -> Self {
        Self {
            event_register,
            ..self
        }
    }
}

#[derive(Debug)]
pub struct ResultCore {
    pub state: State,
    pub ui: Html,
    pub event_register: HashMap<String, Vec<String>>,
}

pub async fn run(modules: HashMap<String, Box<dyn Module>>, opts: CoreOptions) -> ResultCore {
    *STATE.write().await = opts.starting_state;
    *UI.write().await = opts.starting_ui;
    *HANDLER_REGISTER.write().await = opts.event_register;
    *MODULES.write().await = modules;

    let modules = MODULES.read().await;
    for event in opts.events_thrown_pre_init {
        for m in modules.values() {
            m.handler(event.clone(), Box::new(InnerCore)).await;
        }
    }

    let modules = MODULES.read().await;
    for m in modules.values() {
        m.init(Box::new(InnerCore)).await;
    }

    let modules = MODULES.read().await;
    for event in opts.events_thrown_post_init {
        for m in modules.values() {
            m.handler(event.clone(), Box::new(InnerCore)).await;
        }
    }

    ResultCore {
        state: STATE.read().await.clone(),
        ui: UI.read().await.clone(),
        event_register: HANDLER_REGISTER.read().await.clone(),
    }
}
