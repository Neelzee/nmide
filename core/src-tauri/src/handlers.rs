use crate::{
    core::{runtime_core::RuntimeCore, NmideCore},
    statics::{COMPILE_TIME_MODULES, MODULE_EVENT_REGISTER, NMIDE, RUNTIME_MODULES},
};
use abi_stable::sabi_trait::TD_CanDowncast;
use core_module_lib::rs_module::RCore_CTO;
use core_std_lib::{core::Core, core_modification::CoreModification, event::Event};
use foreign_std_lib::event::rs_event::REvent;
use futures;
use log::info;
use tauri::Emitter;

pub async fn init(cm: CoreModification) {
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
    NmideCore
        .get_sender()
        .await
        .send(cm)
        .await
        .expect("Nmide core receiver should not be closed");
    futures::future::join_all(module_futures).await;
    futures::future::join_all(rt_module_futures).await;

    NMIDE
        .get()
        .expect("AppHandle should be initialized at this point")
        .read()
        .await
        .emit("nmide://event", Event::new("post-init", "nmide", None))
        .expect("Emit should succeed")
}

pub async fn handler(event: Event, modifications: Vec<CoreModification>) {
    let event_name = event.event_name().to_string();
    let module = event.module_name().to_string();
    tokio::spawn({
        async move {
            let evt = event.clone();
            let revt = REvent::from(evt.clone());
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
            info!("[backend] Triggered modules: {:?}", triggered_modules);
            for mod_name in triggered_modules {
                if let Some(m) = mods.get(&mod_name) {
                    modules.push(m.handler(evt.clone(), Box::new(NmideCore)));
                }

                if let Some(m) = rt_mods.get(&mod_name) {
                    rt_modules.push(m.handler(revt.clone(), || {
                        RCore_CTO::from_const(&RuntimeCore, TD_CanDowncast)
                    }));
                }
            }
            futures::future::join_all(modules).await;
            futures::future::join_all(rt_modules).await;
        }
    });

    let cm = modifications
        .into_iter()
        .fold(CoreModification::default(), |acc, cm| acc.combine(cm));

    NmideCore
        .get_sender()
        .await
        .send(cm)
        .await
        .expect("Channel should be open");

    if event_name == "nmide://exit" && module == "nmide" {
        let app = NMIDE
            .get()
            .expect("AppHandle should be initialized")
            .read()
            .await;
        info!("[backend][handler] Exiting");
        app.exit(0);
    }
}
