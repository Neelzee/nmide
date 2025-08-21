use anyhow::{Result, anyhow};
use core_module_lib::Module;
use core_std_lib::{core_modification::CoreModification, event::Event, html::Html, state::State};
use empty_module::EmptyModule;
use futures::FutureExt;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, thread::sleep, time::Duration};
use suite::Suite;
use tokio::sync::{self, RwLock, mpsc::Sender, oneshot};
use ts_rs::TS;

mod empty_module;
mod suite;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dependency {
    pub providing: Vec<Event>,
    pub consuming: Vec<(Option<String>, Option<String>)>,
    pub success: bool,
}

impl Dependency {
    pub fn to_named(self, name: String) -> NamedDependency {
        NamedDependency {
            name,
            providing: self.providing,
            consuming: self
                .consuming
                .into_iter()
                .map(|t| Consumer::from_tup(t))
                .collect(),
            success: self.success,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct Consumer {
    event_name: Option<String>,
    module_name: Option<String>,
}

impl Consumer {
    pub fn from_tup((event_name, module_name): (Option<String>, Option<String>)) -> Self {
        Self {
            event_name,
            module_name,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct NamedDependency {
    pub name: String,
    pub providing: Vec<Event>,
    pub consuming: Vec<Consumer>,
    pub success: bool,
}

pub async fn init(module: Box<dyn Module>, dur: Duration) -> Result<Dependency> {
    let mut suite = SUITE.write().await;
    suite.initialize(module).await;
    let (sender, recv) = oneshot::channel::<bool>();
    tokio::spawn(async move {
        let module = MODULE.read().await;
        let mod_init = module.init(Box::new(Core));
        let mod_timeout = async { sleep(dur) };
        let result = futures::select_biased! {
           _ = mod_init.fuse() => true,
           _ = mod_timeout.fuse() => false,
        };
        sender.send(result).expect("Channel should be open");
    });
    match recv.await {
        Ok(success) => {
            if success {
                sleep(dur);
            }
            let providing = THROWN_EVENTS.read().await.clone();
            let consuming = CONSUMER.read().await.clone();
            Ok(Dependency {
                providing,
                consuming,
                success,
            })
        }
        Err(err) => Err(anyhow!(err)),
    }
}

pub struct Core;

pub static STATE: Lazy<RwLock<State>> = Lazy::new(|| RwLock::new(State::default()));
pub static UI: Lazy<RwLock<Html>> = Lazy::new(|| RwLock::new(Html::Main()));
pub static SENDER: sync::OnceCell<Sender<CoreModification>> = sync::OnceCell::const_new();
/// Modules -> Partial Events it consumes on
pub static CONSUMER: Lazy<RwLock<Vec<(Option<String>, Option<String>)>>> =
    Lazy::new(|| RwLock::new(Vec::new()));

pub static MODULE_EVENT_REGISTER: Lazy<RwLock<ModuleEventRegister>> =
    Lazy::new(|| RwLock::new(ModuleEventRegister::default()));

pub static THROWN_EVENTS: Lazy<RwLock<Vec<Event>>> = Lazy::new(|| RwLock::new(Vec::new()));
pub static MODULE: Lazy<RwLock<Box<dyn Module>>> = Lazy::new(|| RwLock::new(Box::new(EmptyModule)));
pub static MODULE_NAME: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));
pub static SUITE: Lazy<RwLock<Suite>> = Lazy::new(|| RwLock::new(Suite::new()));

#[derive(Default, Debug)]
pub struct ModuleEventRegister {
    /// Module -> List of events it reacts too
    event: RwLock<HashMap<String, Vec<String>>>,
    /// Module -> List of modules it reactors too
    module: RwLock<HashMap<String, Vec<String>>>,
}

impl ModuleEventRegister {
    pub async fn get_module_names(&self, event: &Event) -> Vec<String> {
        let mut modules = Vec::new();

        modules.append(
            &mut self
                .event
                .read()
                .await
                .get(event.event_name())
                .cloned()
                .unwrap_or(Vec::new()),
        );
        modules.append(
            &mut self
                .module
                .read()
                .await
                .get(event.module_name())
                .cloned()
                .unwrap_or(Vec::new()),
        );

        modules
    }

    pub async fn register_module(
        &mut self,
        event: Option<String>,
        module: Option<String>,
        handler: String,
    ) {
        if let Some(evt) = event.clone() {
            let mut modules = self.event.write().await;
            let mut vec = modules.get(&evt).cloned().unwrap_or(Vec::new());
            vec.push(handler.clone());
            modules.insert(evt, vec);
        }
        if let Some(md) = module.clone() {
            let mut modules = self.module.write().await;
            let mut vec = modules.get(&md).cloned().unwrap_or(Vec::new());
            vec.push(handler.clone());
            modules.insert(md, vec);
        }
        let m_name = MODULE_NAME.read().await.clone();

        if handler != m_name {
            return;
        }

        let mut cons = CONSUMER.write().await;
        cons.push((event, module))
    }
}

#[async_trait::async_trait]
impl core_std_lib::core::Core for Core {
    async fn state(&self) -> core_std_lib::state::State {
        let state = STATE.read().await;
        state.clone()
    }

    async fn ui(&self) -> core_std_lib::html::Html {
        let ui = UI.read().await;
        ui.clone()
    }

    async fn throw_event(&self, event: core_std_lib::event::Event) {
        let mut te = THROWN_EVENTS.write().await;
        te.push(event.clone());
        drop(te);
        tokio::spawn({
            async move {
                let evt = event.clone();
                let mods = MODULE.read().await;
                let mut modules = Vec::new();
                let triggered_modules = MODULE_EVENT_REGISTER
                    .read()
                    .await
                    .get_module_names(&evt)
                    .await;
                for mod_name in triggered_modules {
                    if mod_name == mods.name() {
                        modules.push(mods.handler(evt.clone(), Box::new(Core)));
                    }
                }
                futures::future::join_all(modules).await;
            }
        });
    }

    async fn add_handler(
        &self,
        event_name: Option<String>,
        module_name: Option<String>,
        handler_name: String,
    ) {
        let mut reg = MODULE_EVENT_REGISTER.write().await;
        reg.register_module(event_name, module_name, handler_name)
            .await;
    }

    async fn get_sender(&self) -> Sender<CoreModification> {
        SENDER.get().unwrap().clone()
    }
}
