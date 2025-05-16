use anyhow::{Result, anyhow};
use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{StateInstructionBuilder, Value},
};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

const MODULE_NAME: &'static str = "ide_explorer";

#[async_trait::async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler("open-project".to_string(), MODULE_NAME.to_string())
            .await;
        core.add_handler("toggle-folder".to_string(), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match event.event_name() {
            "open-project" => {
                let args = event.args();
                if args.is_none() {
                    core.throw_event(error("No argument on open-project event"))
                        .await;
                    return;
                }
                let files = get_args(args.unwrap().clone());
                if files.is_err() {
                    core.throw_event(error(&format!("{:?}", files.unwrap_err())))
                        .await;
                    return;
                }
                let html = render(files.unwrap());
                core.send_modification(
                    CoreModification::default().set_ui(
                        UIInstructionBuilder::default().add_node(
                            Html::Div()
                                .adopt(html)
                                .add_attr(Attr::Class("file-explorer".to_string())),
                            Some("sidebar"),
                        ),
                    ),
                )
                .await;
                core.throw_event(Event::new("open-project-post", None))
                    .await;
            }
            "toggle-folder" => {
                let args = event.args().and_then(|v| v.obj()).unwrap_or_default();
                let id = args.get("eventArgs").and_then(|v| v.str());
                if id.is_none() {
                    return;
                }
                let id = id.unwrap().replace("/", "_");

                let state = core.state().await;

                let collapsed = state.get(&id).and_then(|v| v.bool()).unwrap_or(true);

                let state_builder =
                    StateInstructionBuilder::default().add(id.clone(), Value::Bool(!collapsed));

                if collapsed {
                    core.send_modification(
                        CoreModification::default()
                            .set_ui(
                                UIInstructionBuilder::default()
                                    .rem_attr(Attr::Class("collapsed".to_string()), id),
                            )
                            .set_state(state_builder),
                    )
                    .await;
                } else {
                    core.send_modification(
                        CoreModification::default()
                            .set_ui(
                                UIInstructionBuilder::default()
                                    .add_attr(id, Attr::Class("collapsed".to_string())),
                            )
                            .set_state(state_builder),
                    )
                    .await;
                }
            }
            _ => (),
        }
    }
}

fn render(f: Fo) -> Html {
    match f {
        Fo::File(path) => {
            let file_name = path.split("/").last().unwrap_or(&path);

            Html::Span()
                .set_text(file_name)
                .add_attr(Attr::Id(path.replace("/", "_")))
                .add_attr(Attr::Class("file".to_string()))
        }
        Fo::Folder(path, fos) => {
            let file_name = path.split("/").last().unwrap_or(&path);
            let mut kids =
                vec![
                    Html::Span()
                        .set_text(file_name)
                        .add_attr(Attr::Click(Event::new(
                            "toggle-folder",
                            Some(Value::Str(path.replace("/", "_"))),
                        ))),
                ];
            kids.append(&mut fos.into_iter().map(|k| render(k)).collect());
            Html::Div()
                .add_attr(Attr::Id(path.replace("/", "_")))
                .add_attr(Attr::Class("folder collapsed".to_string()))
                .replace_kids(kids)
        }
    }
}

fn error(s: &str) -> Event {
    Event::new(
        format!("{MODULE_NAME}_ERROR"),
        Some(Value::Str(s.to_string())),
    )
}

#[derive(Debug)]
enum Fo {
    File(String),
    Folder(String, Vec<Fo>),
}

fn get_args(value: Value) -> Result<Fo> {
    match value.obj() {
        Some(obj) => {
            let root = obj
                .get("folder")
                .or_else(|| obj.get("file"))
                .and_then(|v| v.obj())
                .ok_or(anyhow!("Couldn't get root object from argument: {value:?}"))?;

            let path = root
                .get("path")
                .and_then(|v| v.str())
                .ok_or(anyhow!("Couldn't get path from object: {value:?}"))?;

            if let Some(content) = root.get("contents").and_then(|v| v.list()) {
                Ok(Fo::Folder(
                    path,
                    content
                        .clone()
                        .into_iter()
                        .map(|v| get_args(v))
                        .collect::<Result<Vec<_>>>()?,
                ))
            } else {
                Ok(Fo::File(path))
            }
        }
        None => Err(anyhow!("Couldn't get path from argument: {value:?}")),
    }
}
