use core_module_lib::Module;
use core_std_lib::{core::Core as _, core_modification::CoreModification, event::Event, html::Html, state::State};
use once_cell::sync::Lazy;
use std::{collections::{HashMap, HashSet}, time::Duration};
use tokio::sync::{mpsc::{self, Sender}, RwLock};

#[allow(unused_imports)]
pub mod module_reg {
    use core_module_lib::Module;
    use core_module_lib::ModuleBuilder;
    use core_std_lib::core::Core;
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/module_reg.rs"));
}

pub struct Core;

pub static STATE: Lazy<RwLock<State>> = Lazy::new(|| RwLock::new(State::default()));
pub static UI: Lazy<RwLock<Html>> = Lazy::new(|| RwLock::new(Html::Main()));
pub static SENDER: tokio::sync::OnceCell<Sender<CoreModification>> = tokio::sync::OnceCell::const_new();
/// Event -> Modules providing the Event
pub static PROVIDERS: Lazy<RwLock<HashMap<Event, Vec<String>>>> = Lazy::new(|| RwLock::new(HashMap::new()));
/// Modules -> Partial Events it consumes on
pub static CONSUMER: Lazy<RwLock<HashMap<String, Vec<(Option<String>, Option<String>)>>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub static MODULE_EVENT_REGISTER: Lazy<RwLock<ModuleEventRegister>> =
    Lazy::new(|| RwLock::new(ModuleEventRegister::default()));

pub static COMPILE_TIME_MODULES: Lazy<RwLock<HashMap<String, Box<dyn Module>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

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
        let mut cons = CONSUMER.write().await;
        let mut parts = cons.get(&handler).cloned().unwrap_or_default();
        parts.push((event, module));
        cons.insert(handler, parts);
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
        tokio::spawn({
            async move {
                let evt = event.clone();
                let mods = COMPILE_TIME_MODULES.read().await;
                let mut modules = Vec::new();
                let triggered_modules = MODULE_EVENT_REGISTER
                    .read()
                    .await
                    .get_module_names(&evt)
                    .await;
                for mod_name in triggered_modules
                {
                    if let Some(m) = mods.get(&mod_name) {
                        modules.push(m.handler(evt.clone(), Box::new(Core)));
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

#[tokio::main]
async fn main() {
    let mut untriggered_events: HashSet<(Option<String>, Option<String>)> = HashSet::new();
    let mut triggered_events: HashSet<(Option<String>, Option<String>)> = HashSet::new();
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    module_reg::register_modules(&mut modules);
    let mut mods = COMPILE_TIME_MODULES.write().await;
    *mods = modules;
    drop(mods);
    runtime();
    init();

    std::thread::sleep(Duration::from_secs(3));

    let cons = CONSUMER.read().await.clone().clone().clone().clone();

    for p in  cons.values().into_iter().flatten() {
        untriggered_events.insert(p.clone());
    }

    let mut untriggered_events: Vec<_> = untriggered_events.into_iter().collect();

    while let Some((et, md)) = untriggered_events.pop() {
        triggered_events.insert((et.clone(), md.clone()));
        Core.throw_event(Event::new(et.unwrap_or_default(), md.unwrap_or_default(), None)).await;
    }

    register(triggered_events, 1).await;

    println!("Getting data");

    std::thread::sleep(Duration::from_secs(3));

    let cons = CONSUMER.read().await.clone().clone().clone().clone();
    let provs = PROVIDERS.read().await.clone().clone().clone().clone();

    for (mods, deps) in cons {
        println!("Module `{mods}` depends on:");
        for d in deps {
            match d {
                (None, None) => todo!(),
                (None, Some(m)) => println!("Module `{m}`"),
                (Some(e), None) => println!("Event `{e}`"),
                (Some(e), Some(m)) => println!("Event `{e}` from module `{m}`"),
            }
        }
    }

    for (evt, facs) in provs {
        println!("Event `{evt:?}` provided by:");
        for m in facs {
            println!("Module: `{m}`");
        }
    }
}

async fn register(mut triggered_events: HashSet<(Option<String>, Option<String>)>, count: usize) {
    std::thread::sleep(Duration::from_secs(3));
    println!("Starting iteration: {count}");
    let mut untriggered_events: HashSet<(Option<String>, Option<String>)> = HashSet::new();
    let cons = CONSUMER.read().await.clone().clone().clone().clone();

    for p in  cons.values().into_iter().flatten() {
        untriggered_events.insert(p.clone());
    }

    let mut untriggered_events: Vec<_> = untriggered_events.into_iter().filter(|evt| !triggered_events.contains(evt)).collect();

    while let Some((et, md)) = untriggered_events.pop() {
        triggered_events.insert((et.clone(), md.clone()));
        Core.throw_event(Event::new(et.unwrap_or_default(), md.unwrap_or_default(), None)).await;
    }
    println!("Finished iteration");
    std::thread::sleep(Duration::from_secs(3));
    let cons = CONSUMER.read().await.clone().clone().clone().clone();
    let mut untriggered_events: HashSet<(Option<String>, Option<String>)> = HashSet::new();
    for p in  cons.values().into_iter().flatten() {
        untriggered_events.insert(p.clone());
    }

    if untriggered_events.into_iter().filter(|evt| !triggered_events.contains(evt)).count() == 0 {
        println!("No new event sent, exiting");
        return;
    } else {
        Box::pin(register(triggered_events, count + 1)).await;
    }
}


fn runtime() {
    tokio::spawn({
        let (sender, mut recv) = mpsc::channel::<CoreModification>(100);
        SENDER.set(sender).expect("Sender is not set yet");
        async move {
            while let Some(mods) = recv.recv().await {
                let state = Core.state().await;
                let ui = Core.ui().await;

                let (new_state, new_ui) = mods.build(state, ui);

                let mut st = STATE.write().await;
                let mut current_ui = UI.write().await;
                *st = new_state;
                *current_ui = new_ui;
            }
        }
    });
}

fn init() {
    tokio::spawn({
        async move {
            futures::future::join_all(
                COMPILE_TIME_MODULES.read()
                    .await
                    .values()
                    .map(|m| m.init(Box::new(Core)))
            ).await;
        }
    });
}