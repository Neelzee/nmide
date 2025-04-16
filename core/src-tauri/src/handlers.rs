use std::future::IntoFuture;

use core_std_lib::{
    core::{Core},
    html::{Html},
};
use futures;
use log::info;
use tauri::Emitter;
use core_std_lib::attrs::Attr;
use core_std_lib::core::CoreModification;
use core_std_lib::event::Event;
use core_std_lib::instruction::Instruction;
use crate::{
    core::NmideCore,
    statics::{COMPILE_TIME_MODULES, NMIDE_STATE, NMIDE_UI},
};
use crate::ide::NMIDE;
use crate::statics::MODULE_EVENT_REGISTER;

pub async fn init(cm: CoreModification) -> (Instruction<Html>, Instruction<String>, Instruction<Attr>) {
    let modules = COMPILE_TIME_MODULES.read().await;

    let state = NmideCore.state().await;
    let ui = NmideCore.ui().await;

    let module_futures = modules.values().map(|m| m.init(&NmideCore));

    let (new_state, ui_builder) = futures::future::join_all(module_futures)
        .await
        .into_iter()
        .reduce(|acc, ins| acc.combine(ins))
        .unwrap_or_default()
        .combine(cm)
        .build_state(state);

    let mut st = NMIDE_STATE.write().await;
    *st = new_state;
    drop(st);
    let inst = ui_builder.instruction();
    let mut current_ui = NMIDE_UI.write().await;
    *current_ui = ui_builder.build(ui);
    inst
}

pub async fn handler(event: Event, modifications: Vec<CoreModification>) {
    let mods = COMPILE_TIME_MODULES.read().await;
    let module_futures = MODULE_EVENT_REGISTER
        .read()
        .await
        .get_module_names(&event)
        .await
        .into_iter()
        .flat_map(|m| mods.get(&m))
        .map(|m| m.handler(&event, &NmideCore));
    let state = NmideCore.state().await;
    let ui = NmideCore.ui().await;

    let cm = futures::future::join_all(module_futures)
        .await
        .into_iter()
        .fold(CoreModification::default(), |acc, cm| acc.combine(cm))
        .combine(
            modifications.into_iter().fold(
                CoreModification::default(),
                |acc, cm| acc.combine(cm)
            )
        );

    let (new_state, ui_builder) = cm.build_state(state);

    let mut st = NMIDE_STATE.write().await;
    *st = new_state;
    drop(st);
    let app = NMIDE
        .get()
        .expect("AppHandle should be initialized")
        .read()
        .await;
    let inst = ui_builder.instruction();
    let mut current_ui = NMIDE_UI.write().await;
    // TODO: Optimize the instruction set before building
    *current_ui = ui_builder.build(ui);
    // TODO: Do a NoOp check before needlessly re-rendering
    app.emit("nmide://render", inst)
        .expect("AppHandle emit should always succeed");
}