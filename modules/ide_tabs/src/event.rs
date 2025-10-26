use crate::{
    EVENT_CHANGE_TAB, EVENT_CHANGED_TAB, ID_TAB_BTN_CONTAINER, ID_TAB_CONTAINER, SHOW_TAB_CLASS,
    STATE_CURRENT_TAB_KEY, STATE_INITIALIZED, STATE_TAB_STORAGE, STATE_TABS,
    post_init::create_tab_content,
};
use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIBuilder},
    state::{Value, state_builder::StateBuilder},
};

/// Adds the supplied content (Html) to the current tab.
///
/// It expects the given `Event` to contain an `Html`, either directly, or in an
/// _object_ at field `eventArgs`. If the module has not been _initialized_,
/// meaning it has not received a `PostInit` event, it will store the Html in
/// the state, and add it during the initialization stage.
pub async fn add_content_handler(event: Event, core: Box<dyn Core>) {
    if event.args().is_none() {
        return;
    }
    let arg = event.args().unwrap();
    let content = if let Some(h) = arg.html() {
        h
    } else {
        arg.obj()
            .and_then(|o| o.get("eventArgs").cloned())
            .and_then(|o| o.html())
            .unwrap_or_else(|| Html::Div())
    };
    let state = core.state().await;
    let tab_id = state
        .get(STATE_CURRENT_TAB_KEY)
        .and_then(|v| v.int())
        .unwrap_or_default();
    let content_id = format!("tab-id-{}", tab_id);
    if state
        .get(STATE_INITIALIZED)
        .and_then(|v| v.bool())
        .unwrap_or(false)
    {
        core.send_modification(
            CoreModification::default()
                .set_ui(UIBuilder::default().add_node(content, Some(content_id))),
        )
        .await;
    } else {
        core.send_modification(
            CoreModification::default().set_state(
                StateBuilder::default().add(
                    STATE_TAB_STORAGE,
                    Value::new_obj()
                        .obj_add("content", Value::Html(content))
                        .obj_add("id", Value::Str(content_id)),
                ),
            ),
        )
        .await;
    }
}

/// Changes the current tab.
///
/// It expects the given `Event` to contain an `Int`, either directly, or in an
/// _object_ at field `eventArgs`.
///
/// Achieves the _toggling_ by removing and adding `Attr::Class("show-tab")` from
/// the current and new tab respectively.
pub async fn change_handler(event: Event, core: Box<dyn Core>) {
    if event.args().is_none() {
        return;
    }
    let arg = event.args().unwrap();
    let id = if let Some(h) = arg.int() {
        h
    } else {
        arg.obj()
            .and_then(|o| o.get("eventArgs").cloned())
            .and_then(|o| o.int())
            .unwrap_or_default()
    };

    let tab_id = core
        .state()
        .await
        .get(STATE_CURRENT_TAB_KEY)
        .and_then(|v| v.int())
        .unwrap_or_default();
    core.send_modification(
        CoreModification::default()
            .set_state(StateBuilder::default().set(STATE_CURRENT_TAB_KEY, Value::Int(id)))
            .set_ui(
                UIBuilder::default()
                    .rem_attr(
                        Attr::Class(SHOW_TAB_CLASS.to_string()),
                        format!("tab-id-{}", tab_id),
                    )
                    .add_attr(
                        format!("tab-id-{}", id),
                        Attr::Class(SHOW_TAB_CLASS.to_string()),
                    ),
            ),
    )
    .await;
    core.throw_event(Event::new(EVENT_CHANGED_TAB, event.args().cloned()))
        .await;
}

/// Adds a new tab
///
/// The given `Event` can optionally contain an `Str`, either directly, or in an
/// _object_ at field `eventArgs`. This will be the `title` of the tab.
pub async fn tab_add_handler(event: Event, core: Box<dyn Core>) {
    let mut tabs = core
        .state()
        .await
        .get(STATE_TABS)
        .and_then(|v| v.list())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|v| v.obj())
        .map(|o| {
            let id = o
                .get("id")
                .cloned()
                .and_then(|v| v.int())
                .unwrap_or_default();

            let title = o
                .get("title")
                .cloned()
                .and_then(|v| v.str())
                .unwrap_or(id.to_string());

            (id, title)
        })
        .collect::<Vec<(i32, String)>>();
    let max_id = tabs
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|i| i.0)
        .unwrap_or_default();
    let title = event
        .args()
        .and_then(|v| {
            if v.is_str() {
                v.str()
            } else {
                v.obj()
                    .and_then(|o| o.get("eventArgs").cloned())
                    .and_then(|v| v.str())
            }
        })
        .unwrap_or((max_id + 1).to_string());
    tabs.push((max_id + 1, title));
    let (id, title) = tabs.iter().max_by(|(a, _), (b, _)| a.cmp(b)).unwrap();
    let id = *id;
    let title = title.to_string();
    let state = StateBuilder::default().set(
        STATE_TABS,
        Value::List(
            tabs.into_iter()
                .map(|(id, title)| {
                    Value::new_obj()
                        .obj_add("id", Value::Int(id))
                        .obj_add("title", Value::Str(title))
                })
                .collect(),
        ),
    );
    let ui = UIBuilder::default()
        .add_node(create_tab_btn(id, title), Some(ID_TAB_BTN_CONTAINER))
        .add_node(create_tab_content(id), Some(ID_TAB_CONTAINER.to_string()));
    core.send_modification(CoreModification::default().set_state(state).set_ui(ui))
        .await;
    core.throw_event(Event::new(EVENT_CHANGE_TAB, Some(Value::Int(id))))
        .await;
}

pub fn create_tab_btn(id: i32, title: String) -> Html {
    Html::Button()
        .set_text(title)
        .add_attr(Attr::Id(format!("tab-btn-{id}")))
        .add_attr(Attr::Class("tab-btn".to_string()))
        .add_attr(Attr::Click(Event::new(
            EVENT_CHANGE_TAB,
            Some(Value::Int(id)),
        )))
}
