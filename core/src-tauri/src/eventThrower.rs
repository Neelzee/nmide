use core_std_lib::{core::Core, event::REvent, html::rhtml::RHtml, map::rmap::RMap};
use crate::statics::{NMIDE_STATE, NMIDE_UI, NMIDE_MODULES, MODULE_EVENT_REGISTER};

async fn throwEvent(event: REvent) {
    let module_keys = MODULE_EVENT_REGISTER
        .read()
        .await
        .get(event.event_name);

    let mut modules = Vec::new();

    let modules_map = NMIDE_MODULES.read().await;

    for k in module_keys {
        if let Some(m) = modules_map.get(k) {
            modules.push(m)
        }
    }

    // Drops the read-lock
    drop(modules_map);

    let mut mods = Vec::new();

    let core = NMIDE_STATE.read().await;

    for m in modules {
        mods.push(m.handler(&event, &core));
    }

    // Drops the read-lock
    drop(core);

    let mut core = NMIDE_STATE.write().await;

    for mod in mods {
        core = core.apply_modification(mod);
    }
}
