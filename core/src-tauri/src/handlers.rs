use crate::{
    core::NmideCore,
    statics::{COMPILE_TIME_MODULES, MODULE_EVENT_REGISTER, NMIDE},
};
use core_std_lib::{core::Core, core_modification::CoreModification, event::Event};
use futures;
use log::info;

pub async fn init(cm: CoreModification) {
    let modules = COMPILE_TIME_MODULES.read().await;
    let module_futures = modules.values().map(|m| m.init(Box::new(NmideCore)));
    NmideCore
        .get_sender()
        .await
        .send(cm)
        .await
        .expect("Nmide core receiver should not be closed");
    futures::future::join_all(module_futures).await;
}

pub async fn handler(event: Event, modifications: Vec<CoreModification>) {
    let event_name = event.event_name().to_string();
    let module = event.module_name().to_string();
    tokio::spawn({
        async move {
            let evt = event.clone();
            let mods = COMPILE_TIME_MODULES.read().await;
            let mut modules = Vec::new();
            for mod_name in MODULE_EVENT_REGISTER
                .read()
                .await
                .get_module_names(&evt)
                .await
            {
                if let Some(m) = mods.get(&mod_name) {
                    modules.push(m.handler(evt.clone(), Box::new(NmideCore)));
                }
            }
            futures::future::join_all(modules).await;
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
