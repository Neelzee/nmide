use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{StateInstructionBuilder, Value},
};

pub struct Module;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

const MODULE_NAME: &str = "ide_tabs";

const EVENT_ADD_CONTENT: &str = "add_content";

const EVENT_CHANGE_TAB: &str = "change_tab";

const EVENT_ADD_TAB: &str = "add_tab";

const EVENT_REM_TAB: &str = "rem_tab";

const STATE_KEY: &str = "ide-cache.ide_tabs_tabs";

const CURRENT_TAB_KEY: &str = "ide-cache.ide_tabs_current_tab";

const SHOW_TAB_CLASS: &str = "show-tab";

const HIDE_TAB_CLASS: &str = "hide-tab";

#[async_trait::async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(Event::PostInit.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(EVENT_ADD_CONTENT.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler(EVENT_CHANGE_TAB.to_string(), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match event {
            Event::PostInit => {
                let mods = CoreModification::default();
                let state = StateInstructionBuilder::default()
                    .add(
                        STATE_KEY,
                        Value::List(vec![Value::new_obj().obj_add("id", Value::Int(0))]),
                    )
                    .add(CURRENT_TAB_KEY, Value::Int(0));
                let ui = UIInstructionBuilder::default().add_node(
                    Html::Div()
                        .add_attr(Attr::Id("tab-id-0".to_string()))
                        .add_attr(Attr::Class(SHOW_TAB_CLASS.to_string()))
                        .add_attr(Attr::Class(HIDE_TAB_CLASS.to_string())),
                    Some("content"),
                );
                core.get_sender()
                    .await
                    .send(mods.set_state(state).set_ui(ui))
                    .await
                    .expect("Channel should be opened");
            }
            Event::Event {
                event,
                args: Some(arg),
            } if &event == EVENT_ADD_CONTENT => {
                let content = if let Some(h) = arg.html() {
                    h
                } else {
                    arg.obj()
                        .and_then(|o| o.get("eventArgs").cloned())
                        .and_then(|o| o.html())
                        .unwrap_or_else(|| Html::Div())
                };
                let tab_id = core
                    .state()
                    .await
                    .get(CURRENT_TAB_KEY)
                    .cloned()
                    .and_then(|v| v.int())
                    .unwrap_or_default();
                core.get_sender()
                    .await
                    .send(
                        CoreModification::default().set_ui(
                            UIInstructionBuilder::default()
                                .add_node(content, Some(format!("tab-id-{}", tab_id))),
                        ),
                    )
                    .await
                    .expect("Channel should be opened");
            }
            Event::Event {
                event,
                args: Some(arg),
            } if &event == EVENT_CHANGE_TAB => {
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
                    .get(CURRENT_TAB_KEY)
                    .cloned()
                    .and_then(|v| v.int())
                    .unwrap_or_default();

                core.get_sender()
                    .await
                    .send(
                        CoreModification::default()
                            .set_state(
                                StateInstructionBuilder::default()
                                    .set(CURRENT_TAB_KEY, Value::Int(id)),
                            )
                            .set_ui(
                                UIInstructionBuilder::default()
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
                    .await
                    .expect("Channel should be opened");
            }
            Event::Event { event, .. } if &event == EVENT_ADD_TAB => {
                let mut tabs = core
                    .state()
                    .await
                    .get(STATE_KEY)
                    .cloned()
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
                            .unwrap_or(format!("{id}"));

                        (id, title)
                    })
                    .collect::<Vec<(i32, String)>>();
                tabs.sort_by(|a, b| a.0.cmp(&b.0));
                let max_id = tabs
                    .iter()
                    .max_by(|a, b| a.0.cmp(&b.0))
                    .map(|i| i.0)
                    .unwrap_or_default();
                tabs.push((max_id + 1, format!("{}", max_id + 1)));
            }
            _ => (),
        }
    }
}
