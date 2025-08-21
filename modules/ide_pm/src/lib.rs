use std::collections::HashMap;

use core_module_lib::Module;
use core_std_lib::attrs::Attr;
use core_std_lib::core::Core;
use core_std_lib::core_modification::CoreModification;
use core_std_lib::event::{DialogFileKind, Event};
use core_std_lib::html::{Html, UIBuilder};
use core_std_lib::state::{HHMap, Value, state_builder::StateBuilder};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl Module {
        ProjectManagerModule
    }
}

struct ProjectManagerModule;

const MODULE_NAME: &str = "ide_pm";

fn navbar(xs: Vec<(&str, Vec<(&str, Event)>)>) -> Vec<Html> {
    xs.into_iter()
        .map(|(x, ys)| {
            let id = format!("ide-pm-drop-{}", x.replace(" ", "-"));
            Html::Div()
                .add_attr(Attr::Class("dropdown".to_string()))
                .adopt(
                    Html::Button()
                        .set_text(x)
                        .add_attr(Attr::Click(Event::new(
                            "ide-pm-dropdown".to_string(),
                            Some(Value::Str(id.clone().to_lowercase())),
                        )))
                        .add_attr(Attr::Id(id.to_lowercase()))
                        .add_attr(Attr::Class("dropbtn".to_string())),
                )
                .adopt(
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
                                        .add_attr(Attr::Click(Event::new(
                                            "ide-pm-dropdown".to_string(),
                                            Some(Value::Str(id.clone().to_lowercase())),
                                        )))
                                })
                                .collect(),
                        ),
                )
        })
        .collect()
}

#[async_trait::async_trait]
impl Module for ProjectManagerModule {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(Event::PostInit.to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("ide-pm-dropdown".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("ide-pm-file".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("ide-pm-folder".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("nmide://file".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("fsa_read_ide_pm".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("nmide://folder".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("fsa_dir_ide_pm".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("ide-pm-folder-res".to_string(), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match event.event_name() {
            "nmide://post-init" => {
                let mods = UIBuilder::default().add_nodes(
                    navbar(vec![
                        (
                            "File",
                            vec![
                                ("Open File", Event::new("ide-pm-file", None)),
                                ("Open Folder", Event::new("ide-pm-folder", None)),
                                ("Save File", Event::new("ide-save", None)),
                            ],
                        ),
                        (
                            "Edit",
                            vec![("Add Module", Event::new("toggle-add-module", None))],
                        ),
                        ("Selection", vec![]),
                        ("View", vec![("Graph", Event::new("get_graph", None))]),
                    ]),
                    Some("navbar"),
                );
                core.send_modification(CoreModification::ui(mods)).await;
                core.throw_event(Event::new("post-ide-pm", None)).await;
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
                    _ => panic!("Unallowed argument"),
                };
                let id = format!("{id}-content");
                let toggle = !state.get(&id).and_then(|v| v.bool()).is_some_and(|v| v);
                let mods = if toggle {
                    UIBuilder::default().add_attr(id.clone(), Attr::Class("show".to_string()))
                } else {
                    UIBuilder::default().rem_attr(Attr::Class("show".to_string()), id.clone())
                };
                core.send_modification(
                    CoreModification::ui(mods)
                        .set_state(StateBuilder::default().set(id, Value::Bool(toggle))),
                )
                .await;
            }
            "ide-pm-file" => {
                core.throw_event(Event::new("nmide://file?", None)).await;
            }
            "nmide://file" => {
                core.throw_event(Event::new("fsa-read", event.args().cloned()))
                    .await;
            }
            "ide-pm-folder" => {
                tokio::spawn({
                    async move {
                        let builder = Event::new_file_dialog()
                            .event("ide-pm-folder-res")
                            .file_kind(DialogFileKind::SingleDir)
                            .title("Project")
                            .create_dirs(true)
                            .build()
                            .unwrap();
                        core.throw_event(builder).await;
                    }
                });
            }
            "ide-pm-folder-res" | "nmide://folder" => {
                let mut args = event
                    .args()
                    .and_then(|o| o.obj())
                    .or_else(|| {
                        event.args().unwrap().str().map(|s| {
                            let mut map = HashMap::new();
                            map.insert("file_path".to_string(), Value::Str(s));
                            map
                        })
                    })
                    .unwrap_or_default();
                args.insert("module".to_string(), Value::Str("ide_pm".to_string()));
                args.insert("depth".to_string(), Value::Int(5));
                args.insert("ignore_hidden".to_string(), Value::Bool(true));
                core.throw_event(Event::new("fsa-dir", Some(Value::Obj(HHMap::from(args)))))
                    .await;
            }
            "fsa_read_ide_pm" => {
                return;
            }
            "fsa_dir_ide_pm" => {
                core.throw_event(Event::new("open-project", event.args().cloned()))
                    .await;
            }
            _ => (),
        }
    }
}
