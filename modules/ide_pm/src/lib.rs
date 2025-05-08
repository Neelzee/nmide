use core_module_lib::Module;
use core_std_lib::attrs::Attr;
use core_std_lib::core::Core;
use core_std_lib::core_modification::CoreModification;
use core_std_lib::event::Event;
use core_std_lib::html::{Html, UIInstructionBuilder};
use core_std_lib::state::{StateInstructionBuilder, Value};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl Module {
        ProjectManagerModule
    }
}

struct ProjectManagerModule;

const MODULE_NAME: &'static str = "ide_pm";

fn navbar(xs: Vec<(&str, Vec<(&str, Event)>)>) -> Vec<Html> {
    xs.into_iter()
        .map(|(x, ys)| {
            let id = format!("ide-pm-drop-{}", x.replace(" ", "-"));
            (
                Html::Button()
                    .set_text(x)
                    .add_attr(Attr::Click(Event::new(
                        "ide-pm-dropdown".to_string(),
                        MODULE_NAME.to_string(),
                        Some(Value::Str(id.clone().to_lowercase())),
                    )))
                    .add_attr(Attr::Id(id.to_lowercase()))
                    .add_attr(Attr::Class("dropbtn".to_string())),
                Html::Div()
                    .add_attr(Attr::Id(format!("{id}-content").to_lowercase()))
                    .add_attr(Attr::Class("dropdown-content".to_string()))
                    .replace_kids(
                        ys.into_iter()
                            .map(|(y, evt)| {
                                let yid = format!("ide-pm-{}", y.replace(" ", "-"));
                                Html::Button()
                                    .set_text(y)
                                    .add_attr(Attr::Id(yid.to_lowercase()))
                                    .add_attr(Attr::Click(evt))
                            })
                            .collect(),
                    ),
            )
        })
        .flat_map(|(b, c)| vec![b, c])
        .collect()
}

#[async_trait::async_trait]
impl Module for ProjectManagerModule {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(Some("post-init".to_string()), None, MODULE_NAME.to_string())
            .await;
        core.add_handler(
            Some("ide-pm-dropdown".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        core.add_handler(
            Some("ide-pm-file".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        core.add_handler(
            Some("ide-pm-folder".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        core.add_handler(
            Some("nmide://file".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        core.add_handler(
            Some("fsa_read_ide_pm".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        core.add_handler(
            Some("nmide://folder".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
        core.add_handler(
            Some("fsa_dir_ide_pm".to_string()),
            None,
            MODULE_NAME.to_string(),
        )
        .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        let sender = core.get_sender().await;
        match event.event_name() {
            "post-init" => {
                let mods = UIInstructionBuilder::default().add_nodes(
                    navbar(vec![
                        (
                            "File",
                            vec![
                                ("Open File", Event::new("ide-pm-file", MODULE_NAME, None)),
                                (
                                    "Open Folder",
                                    Event::new("ide-pm-folder", MODULE_NAME, None),
                                ),
                            ],
                        ),
                        ("Edit", vec![]),
                        ("Selection", vec![]),
                        (
                            "View",
                            vec![("Graph", Event::new("get_magnolia_graph", MODULE_NAME, None))],
                        ),
                    ]),
                    Some("navbar"),
                );
                sender
                    .send(CoreModification::ui(mods))
                    .await
                    .expect("Channel should be open");
            }
            "ide-pm-dropdown" if event.args().is_some() => {
                let state = core.state().await;
                let id = match event.args().unwrap() {
                    Value::Str(s) => s.to_string(),
                    Value::Obj(obj) => obj
                        .clone()
                        .to_hm()
                        .get("eventArgs")
                        .cloned()
                        .and_then(|v| v.str())
                        .unwrap(),
                    _ => panic!("Unhallowed argument"),
                };
                let id = format!("{id}-content");
                let toggle = !state.get(&id).and_then(|v| v.bool()).is_some_and(|v| v);
                let mods = if toggle {
                    UIInstructionBuilder::default()
                        .add_attr(id.clone(), Attr::Class("show".to_string()))
                } else {
                    UIInstructionBuilder::default()
                        .rem_attr(Attr::Class("show".to_string()), id.clone())
                };
                sender
                    .send(
                        CoreModification::ui(mods).set_state(
                            StateInstructionBuilder::default().set(id, Value::Bool(toggle)),
                        ),
                    )
                    .await
                    .expect("Channel should be open");
            }
            "ide-pm-file" => {
                core.throw_event(Event::new("nmide://file?", MODULE_NAME, None))
                    .await;
            }
            "nmide://file" => {
                core.throw_event(Event::new("fsa-read", MODULE_NAME, event.args().cloned()))
                    .await;
            }
            "ide-pm-folder" => {
                tokio::spawn({
                    async move {
                        core.throw_event(Event::new("nmide://folder?", MODULE_NAME, None))
                            .await;
                    }
                });
            }
            "nmide://folder" => {
                core.throw_event(Event::new("fsa-dir", MODULE_NAME, event.args().cloned()))
                    .await;
            }
            "fsa-read-ide_pm" => {
                println!("DATA: {:?}", event.args());
            }
            "fsa-dir-ide_pm" => {
                println!("DIR-DATA: {:?}", event.args());
            }
            _ => (),
        }
    }
}
