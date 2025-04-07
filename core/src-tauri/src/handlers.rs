use std::future::IntoFuture;

use anyhow::Context;
use core_std_lib::{
    core::{Core, CoreModification},
    html::{Html, UIInstruction},
};
use futures;
use serde::{Deserialize, Serialize};

use crate::{
    core::{NmideCore, ReturnType},
    statics::{COMPILE_TIME_MODULES, NMIDE_STATE, NMIDE_UI},
};

pub async fn init() -> ReturnType {
    let modules = COMPILE_TIME_MODULES.read().await;

    let state = NmideCore.state().await;
    let ui = NmideCore.ui().await;

    let module_futures = modules.values().map(|m| m.init(&NmideCore));

    let (new_state, ui_builder) = futures::future::join_all(module_futures)
        .await
        .into_iter()
        .reduce(|acc, ins| acc.combine(ins))
        .unwrap_or_default()
        .build_state(state);

    let mut st = NMIDE_STATE.write().await;
    *st = new_state;
    drop(st);
    let inst = ui_builder.get_instructions();
    let mut current_ui = NMIDE_UI.write().await;
    *current_ui = ui_builder.build(ui);
    ReturnType {
        inst,
        ui: current_ui.clone(),
    }
}
