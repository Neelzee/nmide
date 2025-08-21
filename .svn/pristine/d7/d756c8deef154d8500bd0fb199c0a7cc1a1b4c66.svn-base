use abi_stable::sabi_trait::TD_CanDowncast;
use core_module_lib::rs_module::RCore_CTO;
use core_std_lib::{event::Event, state::Value};
use foreign_std_lib::event::rs_event::REvent;
use log::{info, warn};
use tauri_plugin_cli::Matches;

use crate::{
    core::{runtime_core::RuntimeCore, NmideCore},
    statics::{COMPILE_TIME_MODULES, MODULE_EVENT_REGISTER, RUNTIME_MODULES},
};

struct CLIArgs {
    event_names: Vec<String>,
    args: Vec<Option<Value>>,
    exit: bool,
    init: bool,
    force: bool,
    modules: Vec<String>,
}

fn parse_args(matches: Matches) -> CLIArgs {
    info!("Args: {matches:?}");
    let event_names: Vec<String> = matches
        .args
        .get("event")
        .and_then(|a| a.value.as_array())
        .map(|xs| {
            xs.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let exit = matches
        .args
        .get("exit")
        .map(|a| a.occurrences > 0)
        .unwrap_or(false);
    let init = matches
        .args
        .get("init")
        .map(|a| a.occurrences > 0)
        .unwrap_or(false);
    let force = matches
        .args
        .get("force")
        .map(|a| a.occurrences > 0)
        .unwrap_or(false);
    let modules = matches
        .args
        .get("module")
        .and_then(|a| a.value.as_array())
        .map(|xs| {
            xs.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let primitive = matches
        .args
        .get("primitive")
        .and_then(|a| a.value.as_bool())
        .unwrap_or(false);
    let args = matches
        .args
        .get("args")
        .and_then(|a| a.value.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|v| {
            if primitive {
                let val: Result<Value, _> = serde_json::from_str(v.as_str().unwrap_or_default());
                return val.ok();
            }
            fn parse(x: serde_json::Value) -> Option<Value> {
                match x {
                    serde_json::Value::Null => Some(Value::Null),
                    serde_json::Value::Bool(b) => Some(Value::Bool(b)),
                    serde_json::Value::Number(number) if number.is_i64() => {
                        // WARN: This is an unsafe casting!
                        Some(Value::Int(number.as_i64().unwrap_or_default() as i32))
                    }
                    serde_json::Value::Number(number) => {
                        // WARN: This is an unsafe casting!
                        Some(Value::new_float(number.as_f64().unwrap_or_default() as f32))
                    }
                    serde_json::Value::String(s) if s.trim() == "!" => None,
                    serde_json::Value::String(s) => Some(Value::Str(s)),
                    serde_json::Value::Array(values) => {
                        Some(Value::List(values.into_iter().filter_map(parse).collect()))
                    }
                    serde_json::Value::Object(map) => {
                        Some(map.into_iter().fold(Value::new_obj(), |acc, (key, val)| {
                            acc.add(key, parse(val).unwrap_or_default())
                        }))
                    }
                }
            }
            parse(v)
        })
        .collect::<Vec<_>>();

    CLIArgs {
        event_names,
        args,
        exit,
        init,
        force,
        modules,
    }
}

pub async fn run(matches: Matches) {
    let opts = parse_args(matches);
    let init = opts.init;
    let module_names = opts.modules;
    if init && module_names.is_empty() {
        crate::handlers::init().await;
    } else if init {
        let rt_modules = RUNTIME_MODULES
            .get()
            .expect("Should be initialized at this point")
            .read()
            .await;
        let rt_module_futures = rt_modules
            .iter()
            .filter(|(k, _)| module_names.contains(k))
            .map(|(_, m)| m.init(|| RCore_CTO::from_const(&RuntimeCore, TD_CanDowncast)))
            .collect::<Vec<_>>();
        let modules = COMPILE_TIME_MODULES.read().await;
        let module_futures = modules
            .iter()
            .filter(|(k, _)| module_names.contains(k))
            .map(|(_, m)| m.init(Box::new(NmideCore)));
        futures::future::join_all(module_futures).await;
        futures::future::join_all(rt_module_futures).await;
    }
    let events = opts.event_names;
    let args = opts.args;
    let exit = opts.exit;
    if args.len() > events.len() {
        warn!("More Event Arguments given than Events!");
    }
    let mut events = events
        .into_iter()
        .zip(args)
        .map(|(e, a)| Event::new(e, a))
        .collect::<Vec<Event>>();
    if exit {
        events.push(Event::pre_exit());
    }
    events.insert(0, Event::post_init());
    let force = opts.force;

    for e in events {
        if module_names.is_empty() {
            if force {
                let rt_modules = RUNTIME_MODULES
                    .get()
                    .expect("Should be initialized at this point")
                    .read()
                    .await;
                let rt_module_futures = rt_modules
                    .iter()
                    .map(|(_, m)| {
                        m.handler(REvent::from(e.clone()), || {
                            RCore_CTO::from_const(&RuntimeCore, TD_CanDowncast)
                        })
                    })
                    .collect::<Vec<_>>();
                let modules = COMPILE_TIME_MODULES.read().await;
                let module_futures = modules
                    .iter()
                    .map(|(_, m)| m.handler(e.clone(), Box::new(NmideCore)));
                futures::future::join_all(module_futures).await;
                futures::future::join_all(rt_module_futures).await;
                continue;
            }
            crate::handlers::handler(e).await;
        } else {
            let mods = COMPILE_TIME_MODULES.read().await;
            let rt_mods = RUNTIME_MODULES
                .get()
                .expect("Should be initialized")
                .read()
                .await;
            if force {
                let rt_module_futures = rt_mods
                    .iter()
                    .filter(|(k, _)| module_names.contains(k))
                    .map(|(_, m)| {
                        m.handler(REvent::from(e.clone()), || {
                            RCore_CTO::from_const(&RuntimeCore, TD_CanDowncast)
                        })
                    })
                    .collect::<Vec<_>>();
                let module_futures = mods
                    .iter()
                    .filter(|(k, _)| module_names.contains(k))
                    .map(|(_, m)| m.handler(e.clone(), Box::new(NmideCore)));
                futures::future::join_all(module_futures).await;
                futures::future::join_all(rt_module_futures).await;
                continue;
            }
            let mut modules = Vec::new();
            let mut rt_modules = Vec::new();
            let triggered_modules = MODULE_EVENT_REGISTER
                .read()
                .await
                .get_module_names(&e)
                .await;
            for mod_name in triggered_modules {
                if let Some(m) = mods.get(&mod_name) {
                    modules.push(m.handler(e.clone(), Box::new(NmideCore)));
                }

                if let Some(m) = rt_mods.get(&mod_name) {
                    rt_modules.push(m.handler(REvent::from(e.clone()), || {
                        RCore_CTO::from_const(&RuntimeCore, TD_CanDowncast)
                    }));
                }
            }
            futures::future::join_all(modules).await;
            futures::future::join_all(rt_modules).await;
        }
    }
}
