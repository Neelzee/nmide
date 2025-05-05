use async_trait::async_trait;
use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{State, StateInstructionBuilder, Value},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub async fn update(
    event: &Event,
    model: &State,
    core: &Box<dyn Core>,
) -> Result<StateInstructionBuilder, Event> {
    match (
        event.event_name(),
        event.args().cloned().unwrap_or_default(),
    ) {
        ("fsa-read", obj) => {
            let file = if event.args().is_some_and(|s| s.is_str()) {
                event.args().unwrap().str().unwrap()
            } else {
                obj.obj().unwrap().get("eventArgs").unwrap().str().unwrap()
            };
            match read_file(&file, model) {
                Ok(data) => {
                    let evt = Event::new(
                        format!("fsa-read-{}", event.module_name()),
                        module_name().to_string(),
                        Some(Value::Str(data.clone())),
                    );
                    println!("EVENT: {:?}", &evt);
                    core.throw_event(evt).await;
                    Ok(State::build())
                }
                Err(err) => Err(error(err, event)),
            }
        }
        ("fsa-write", Value::Obj(obj)) => match write_file(&obj.to_hm(), model) {
            Ok(file) => Ok(State::build().add(format!("fsa-write-{}", file), Value::Bool(true))),
            Err(err) => Err(error(err, event)),
        },
        ("fsa-dir", obj) if obj.is_obj() => {
            let file = obj.obj().unwrap().get("eventArgs").and_then(|v| v.str()).unwrap_or_default();
            match walk_dir(&file) {
                Ok(data) => {
                    if Path::new(&file).is_dir() {
                        core.throw_event(
                            Event::new(
                                format!("fsa-dir-{}", event.module_name()),
                                module_name().to_string(),
                                Some(Value::new_obj()
                                    .obj_add("folder", Value::Str(file.clone()))
                                    .obj_add(
                                    "contents",
                                    Value::List(
                                        data.clone().into_iter().map(|(b, s)| {
                                            let obj = Value::new_obj().obj_add("path", Value::Str(s));
                                            if b {
                                                Value::new_obj().obj_add("folder", obj.obj_add("contents", Value::List(Vec::new())))
                                            } else {
                                                Value::new_obj().obj_add("fiel", obj)
                                            }
                                        }).collect()
                                    )
                                ))
                            )
                        ).await;
                    } else {
                        core.throw_event(
                            Event::new(
                                format!("fsa-dir-{}", event.module_name()),
                                module_name().to_string(),
                                Some(Value::new_obj()
                                    .obj_add("file", Value::Str(file.clone()))
                                    )
                            )
                        ).await;
                    }
                    Ok(State::build().add(
                        &format!("fsa-dir-{}", file.to_string()),
                        Value::List(data.into_iter().map(|(_, d)| Value::Str(d)).collect()),
                    ))
                },
                Err(err) => Err(error(err, event)),
            }
        },
        _ => Err(error(event_error("invalid event format"), event)),
    }
}

fn event_error<S: ToString>(value: S) -> Event {
    let mut map = HashMap::new();
    map.insert("error".to_string(), Value::Str(value.to_string()));
    Event::new("fsa-error", module_name(), Some(map.into()))
}

fn read_file(key: &str, model: &State) -> Result<String, Event> {
    match model.get(key) {
        Some(val) => {
            let path: PathBuf = val
                .clone()
                .str()
                .ok_or(event_error("Could not match GUID to file"))?
                .to_string()
                .into();
            let mut file = File::open(path).map_err(|err| event_error(format!("{err:?}")))?;
            let mut buf = String::new();
            file.read_to_string(&mut buf)
                .map_err(|err| event_error(format!("{err:?}")))?;
            Ok(buf)
        }
        None => Err(event_error(format!("Could not find: {key} in model"))),
    }
}

fn write_file(obj: &HashMap<String, Value>, model: &State) -> Result<String, Event> {
    let key = obj
        .get("fsa-file")
        .cloned()
        .ok_or(event_error("Could not find `fsa-file` in msg-obj"))?
        .str()
        .ok_or(event_error("Expected `fsa-file` in event-obj to be string"))?;

    let buf = obj
        .get("fsa-content")
        .cloned()
        .ok_or(event_error("Could not find `fsa-content` in msg-obj"))?
        .str()
        .ok_or(event_error(
            "Expected `fsa-content` in msg-obj to be string",
        ))?;

    match model.get(&key) {
        Some(val) => {
            let path: PathBuf = val
                .clone()
                .str()
                .ok_or(event_error("Could not match GUID to file"))?
                .to_string()
                .into();
            let mut file = File::open(path).map_err(|err| event_error(format!("{err:?}")))?;
            file.write_all(buf.as_bytes())
                .map_err(|err| event_error(format!("{err:?}")))?;
            Ok(key.to_string())
        }
        None => Err(event_error(format!("Could not find: {key} in model"))),
    }
}

fn walk_dir(file: &str) -> Result<Vec<(bool, String)>, Event> {
    let path: PathBuf = file
        .to_string()
        .into();
    Ok(walk(&path, true)?
        .into_iter()
        .map(|f| (f.is_dir(), f.to_string_lossy().to_string()))
        .collect())

}

fn walk(pth: &Path, ignore_hidden: bool) -> Result<Vec<PathBuf>, Event> {
    if ignore_hidden && pth.starts_with(".") {
        return Ok(Vec::new());
    }
    if pth.is_dir() {
        Ok(pth
            .read_dir()
            .map_err(|err| event_error(format!("{:?}", err)))?
            .flat_map(|f| {
                f.map(|d| walk(&d.path(), ignore_hidden).unwrap_or_default())
                    .unwrap_or_default()
            })
            .collect())
    } else {
        Ok(vec![pth.to_path_buf()])
    }
}

fn error(error_event: Event, original_event: &Event) -> Event {
    let mut map = error_event
        .args()
        .cloned()
        .unwrap_or_default()
        .obj()
        .unwrap_or_default();
    map.insert(
        "event".to_string(),
        Value::Str(original_event.event_name().to_string()),
    );
    map.insert(
        "value".to_string(),
        original_event.args().cloned().unwrap_or_default(),
    );
    Event::new("fsa-error", module_name(), Some(map.into()))
}

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

const fn module_name() -> &'static str {
    "ide-fsa"
}

#[async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        module_name()
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(
            Some("fsa-write".to_string()),
            None,
            module_name().to_string(),
        )
        .await;
        core.add_handler(
            Some("fsa-read".to_string()),
            None,
            module_name().to_string(),
        )
        .await;
        core.add_handler(Some("fsa-dir".to_string()), None, module_name().to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        let state = core.state().await;
        match update(&event, &state, &core).await {
            Ok(st) => core
                .get_sender()
                .await
                .send(CoreModification::default().set_state(st))
                .await
                .expect("Channel should be opened"),
            Err(err_event) => core.throw_event(err_event).await,
        }
    }
}
