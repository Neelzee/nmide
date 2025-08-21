use async_trait::async_trait;
use core_std_lib::{
    attrs::Attr,
    core::{Core, CoreModification},
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

fn err_combine(l: Value, r: Value) -> Value {
    match l {
        Value::List(mut lst) => {
            lst.push(r);
            Value::List(lst)
        }
        _ => Value::List(vec![r]),
    }
}

pub fn update(event: &Event, model: &State) -> Result<StateInstructionBuilder, Event> {
    match (
        event.event_name(),
        event.args().cloned().unwrap_or_default(),
    ) {
        ("fsa-read", Value::Str(file)) => match read_file(&file, model) {
            Ok(data) => {
                Ok(State::build().add(format!("fsa-read-{}", file.to_string()), Value::Str(data)))
            }
            Err(err) => Err(error(err, event)),
        },
        ("fsa-write", Value::Obj(obj)) => match write_file(&obj, model) {
            Ok(file) => Ok(State::build().add(format!("fsa-write-{}", file), Value::Bool(true))),
            Err(err) => Err(error(err, event)),
        },
        ("fsa-dir", Value::Str(file)) => match walk_dir(&file, model) {
            Ok(data) => Ok(State::build().add(
                &format!("fsa-dir-{}", file.to_string()),
                Value::List(data.into_iter().map(|d| Value::Str(d)).collect()),
            )),
            Err(err) => Err(error(err, event)),
        },
        _ => Err(error(event_error("invalid event format"), event)),
    }
}

fn event_error<S: ToString>(value: S) -> Event {
    let mut map = HashMap::new();
    map.insert("error".to_string(), Value::Str(value.to_string()));
    Event::new("fsa-error", module_name(), Some(Value::Obj(map)))
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

fn walk_dir(key: &str, model: &State) -> Result<Vec<String>, Event> {
    match model.get(key) {
        Some(val) => {
            let path: PathBuf = val
                .clone()
                .str()
                .ok_or(event_error("Could not match GUID to file"))?
                .to_string()
                .into();
            Ok(walk(&path, true)?
                .into_iter()
                .map(|f| f.to_string_lossy().to_string())
                .collect())
        }
        None => Err(event_error(format!("Could not find: {key} in model"))),
    }
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
    Event::new("fsa-error", module_name(), Some(Value::Obj(map)))
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

    async fn init(&self, core: &dyn Core) -> CoreModification {
        core.add_handler(
            Some("fsa-write".to_string()),
            None,
            "trivial_module".to_string(),
        )
        .await;
        core.add_handler(
            Some("fsa-read".to_string()),
            None,
            "trivial_module".to_string(),
        )
        .await;
        core.add_handler(
            Some("fsa_dir".to_string()),
            None,
            "trivial_module".to_string(),
        )
        .await;
        CoreModification::default()
    }

    async fn handler(&self, event: &Event, core: &dyn Core) -> CoreModification {
        let state = core.state().await;
        match update(event, &state) {
            Ok(st) => CoreModification::default().set_state(st),
            Err(err_event) => {
                core.throw_event(err_event).await;
                CoreModification::default()
            }
        }
    }
}
