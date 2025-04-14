use std::future::IntoFuture;

use core_std_lib::{
    core::{Core},
    html::{Html},
};
use futures;
use core_std_lib::attrs::Attr;
use core_std_lib::core::CoreModification;
use core_std_lib::instruction::Instruction;
use crate::{
    core::NmideCore,
    statics::{COMPILE_TIME_MODULES, NMIDE_STATE, NMIDE_UI},
};

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
