use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    html::{Html, UIInstructionBuilder},
    state::{StateInstructionBuilder, Value},
};

use crate::{
    HIDE_TAB_CLASS, ID_TAB_BTN_CONTAINER, ID_TAB_CONTAINER, SHOW_TAB_CLASS, STATE_CURRENT_TAB_KEY,
    STATE_INITIALIZED, STATE_TAB_STORAGE, STATE_TABS, event::create_tab_btn,
};

pub async fn handler(core: Box<dyn Core>) {
    let builder = UIInstructionBuilder::default().add_node(
        Html::Div()
            .add_attr(Attr::Id(ID_TAB_CONTAINER.to_string()))
            .adopt(
                Html::Div()
                    .add_attr(Attr::Id(ID_TAB_BTN_CONTAINER.to_string()))
                    .adopt(create_tab_btn(0, "0".to_string()))
                    .adopt(create_tab_btn(1, "1".to_string())),
            )
            .adopt(create_tab_content(0).add_attr(Attr::Class(SHOW_TAB_CLASS.to_string())))
            .adopt(create_tab_content(1)),
        Some("content"),
    );
    let ui = if let Some(xs) = core
        .state()
        .await
        .get(STATE_TAB_STORAGE)
        .and_then(|v| v.list())
    {
        println!("\n{xs:?}\n");
        xs.into_iter()
            .filter_map(|v| v.obj())
            .filter_map(|o| {
                match (
                    o.get("id").and_then(|v| v.str()),
                    o.get("content").and_then(|v| v.html()),
                ) {
                    (Some(_), None) | (None, Some(_)) | (None, None) => None,
                    (Some(id), Some(html)) => Some((id, html)),
                }
            })
            .fold(builder, |build, (id, html)| build.add_node(html, Some(id)))
    } else {
        builder
    };
    let mods = CoreModification::default();
    let state = StateInstructionBuilder::default()
        .add(
            STATE_TABS,
            Value::List(vec![
                Value::new_obj().obj_add("id", Value::Int(0)),
                Value::new_obj().obj_add("id", Value::Int(1)),
            ]),
        )
        .add(STATE_CURRENT_TAB_KEY, Value::Int(0))
        .set(STATE_INITIALIZED, Value::Bool(true));
    core.send_modification(mods.set_state(state).set_ui(ui))
        .await;
}

pub fn create_tab_content(id: i32) -> Html {
    Html::Div()
        .add_attr(Attr::Id(format!("tab-id-{}", id)))
        .add_attr(Attr::Class(HIDE_TAB_CLASS.to_string()))
}
