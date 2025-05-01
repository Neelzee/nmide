//! Rust Module Grapher
//!
//! Analyses Modules, creating a consumer and provider dependency graph, based
//! on the Event system.
//!
//! A Module has two states, initialization, and handling.
//!
//! ## Initialization
//!
//! During initialization, a Module cant have any dependency on other Modules,
//! but it can register for an Event, meaning the Module will be invoked once
//! the specified Event is triggered, or it can throw an Event.
//!
//! Registration for an Event can always be directly analysed, as it happens on
//! the Core instance. The way a registration occurs, is that a Module supplies
//! an optional string for a specific Event name, and/or Module provider to
//! trigger on, meaning after the first initialization, the consumer graph looks
//! like this: `Module -> Partial Event`.
//!
//! To find the providers, we can initialize a Module, and see what possible
//! Events, if any are thrown. If an Event is thrown, then that Module is a
//! provider of that Event. The mapping between Event and Module is not unique,
//! meaning several Module can provide the same Event. Another way a Module can
//! provide an Event, is through user interaction. A Module can create some UI,
//! which a user can interact with, which would trigger an Event. The UI can be
//! analysed for such triggers, and a mapping between that Event and Module
//! would be created.
//!
//! ## Handler
//!
//! During a handling of a triggered Event, a Module could registrate for new
//! Events, trigger another Event, or do nothing.
//!
//! To find more dependencies, a Module is triggered with the Events it
//! subscribed to, and analysed for new registrations or triggers. If the list
//! all subscribed Events have been exhausted, and no new registration or
//! trigger has occurred, that Module is considered `Exhausted`, and won't be
//! analyzed any more.
//!
//! ## Deadlocks
//!
//! Since Modules are asynchronous, and can possible spawn own threads, there is
//! a possibility for deadlocks to occur. There is no way to avoid this, without
//! restricting Module developers, which is not wanted. Therefore it is up to
//! the user of rsm-grapher to avoid this, by supplying a timeout value,
//! ensuring that if, after some time N, the analysation of a Module is still
//! occurring, the analysation is killed.
//!
//! ## State dependency
//!
//! Modules could be depending on a certain state, before triggering or
//! subscribing to an Event, this is not really possible to know without doing
//! static code analasys, so this is out of scope for this application.

use core_module_lib::Module;
use core_std_lib::event::Event;
use rsm_handler::fold_deps;
use rsm_invoker::Dependency;
use std::{collections::HashMap, time::Duration};

#[allow(unused_imports)]
pub mod module_reg {
    use core_module_lib::Module;
    use core_module_lib::ModuleBuilder;
    use core_std_lib::core::Core;
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/module_reg.rs"));
}

#[tokio::main]
async fn main() {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    module_reg::register_modules(&mut modules);

    let mut map = HashMap::new();
    for (m, module) in modules {
        println!("Analysing initialization of `{m}`");
        match rsm_invoker::init(module, Duration::from_secs(5)).await {
            Ok(deps) => {
                map.insert(m, deps);
            }
            Err(err) => eprintln!("Error during analysing: {err:?}"),
        }
    }

    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    module_reg::register_modules(&mut modules);

    for (m, module) in modules {
        let dep = map.get(&m).unwrap().clone();
        let initial_events = dep
            .consuming
            .clone()
            .into_iter()
            .map(|(evt_name, mod_name)| {
                Event::new(
                    evt_name.unwrap_or_default(),
                    mod_name.unwrap_or_default(),
                    None,
                )
            })
            .collect();
        println!("Analysing event handling of `{m}`");
        match handling(module, m, initial_events, dep).await {
            Ok(Ok(dep)) => println!("{dep:?}"),
            Ok(_) => unreachable!(),
            Err(err) => eprintln!("{}", err),
        }
    }
}

async fn handling(
    module: Box<dyn Module>,
    name: String,
    initial_events: Vec<Event>,
    dep: Dependency,
) -> Result<std::result::Result<Dependency, Vec<Event>>, String> {
    match rsm_handler::handle(module, initial_events, Duration::from_secs(5)).await {
        Ok(Ok(new_dep)) => {
            let dep = fold_deps(dep, new_dep);
            Ok(Ok(dep))
        }
        Ok(Err((dep, new_events))) => {
            println!("Found new events for {name}");
            let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
            module_reg::register_modules(&mut modules);
            let (_, module) = modules.into_iter().find(|m| m.0 == name).unwrap();
            Box::pin(handling(
                module,
                name,
                new_events.into_iter().collect(),
                dep,
            ))
            .await
        }
        Err(err) => Err(format!("{err:?}")),
    }
}
