use crate::{
    context::{compile_time::NmideCore, runtime::RuntimeCore},
    core::statics::{COMPILE_TIME_MODULES, MODULE_EVENT_REGISTER, NMIDE, RUNTIME_MODULES},
};
use abi_stable::sabi_trait::TD_CanDowncast;
use core_module_lib::rs_module::RCore_CTO;
use core_std_lib::{core::Core, event::Event};
use foreign_std_lib::event::rs_event::REvent;
use futures;
use log::info;

pub async fn init() {
    let rt_modules = RUNTIME_MODULES
        .get()
        .expect("Should be initialized at this point")
        .read()
        .await;
    let rt_module_futures = rt_modules
        .values()
        .map(|m| m.init(|| RCore_CTO::from_const(&RuntimeCore, TD_CanDowncast)))
        .collect::<Vec<_>>();
    let modules = COMPILE_TIME_MODULES.read().await;
    let module_futures = modules.values().map(|m| m.init(Box::new(NmideCore)));
    futures::future::join_all(module_futures).await;
    futures::future::join_all(rt_module_futures).await;

    NmideCore.throw_event(Event::PostInit).await;
}

pub async fn handler(event: Event) {
    let evt = event.clone();
    tokio::spawn({
        async move {
            let evt = event.clone();
            let mut revt = None;
            let mods = COMPILE_TIME_MODULES.read().await;
            let rt_mods = RUNTIME_MODULES
                .get()
                .expect("Should be initialized")
                .read()
                .await;
            let mut modules = Vec::new();
            let mut rt_modules = Vec::new();
            let triggered_modules = MODULE_EVENT_REGISTER
                .read()
                .await
                .get_module_names(&evt)
                .await;
            info!(place = "backend", event:serde, triggered_modules:serde; "Handler, {:?} {:?}", event, triggered_modules);
            for mod_name in triggered_modules {
                if let Some(m) = mods.get(&mod_name) {
                    modules.push(m.handler(evt.clone(), Box::new(NmideCore)));
                }

                if let Some(m) = rt_mods.get(&mod_name) {
                    if revt.is_none() {
                        revt = Some(REvent::from(evt.clone()));
                    }
                    rt_modules.push(m.handler(revt.clone().unwrap(), || {
                        RCore_CTO::from_const(&RuntimeCore, TD_CanDowncast)
                    }));
                }
            }
            futures::future::join_all(modules).await;
            futures::future::join_all(rt_modules).await;
        }
    });

    if matches!(evt, Event::PreExit) {
        info!(place = "backend"; "Exiting");
        NMIDE
            .get()
            .expect("AppHandle should be initialized")
            .exit()
            .await;
    }
}
