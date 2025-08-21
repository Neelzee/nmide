use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use core_std_lib::{core::Core, event::Event, state::Value};
use fsa::{walk_dir, FOptions, Fo};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

#[cfg(test)]
mod test_fsa;

mod fsa;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

const fn module_name() -> &'static str {
    "ide_fsa"
}

#[async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        module_name()
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler("fsa-write".to_string(), module_name().to_string())
            .await;
        core.add_handler("fsa-read".to_string(), module_name().to_string())
            .await;
        core.add_handler("fsa-dir".to_string(), module_name().to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        let result = match event.event_name() {
            "fsa-write" => fsa_write(&event, &core).await,
            "fsa-read" => fsa_read(&event, &core).await,
            "fsa-dir" => fsa_dir(&event, &core).await,
            _ => Ok(()),
        };

        if result.is_ok() {
            return;
        }

        let obj = Value::new_obj()
            .obj_add("error_event", Value::Str(event.event_name().to_string()))
            .obj_add("error_args", event.args().cloned().unwrap_or_default())
            .obj_add(
                "error_msg",
                Value::Str(format!("{:?}", result.unwrap_err())),
            );

        core.throw_event(Event::new("fsa-error", Some(obj))).await;
    }
}

async fn fsa_write(event: &Event, core: &Box<dyn Core>) -> Result<()> {
    let arg = event
        .args()
        .ok_or(anyhow!("Expected argument, got nothing"))?
        .obj()
        .ok_or(anyhow!(
            "Expected argument to be of type Object, but got: {:?}",
            event.args()
        ))?;

    let file_path: PathBuf = arg
        .get("file_path")
        .ok_or(anyhow!("Expected object to contain `file_path`: {:?}", arg))?
        .str()
        .ok_or_else(|| {
            anyhow!(
                "Expected `file_path` to be of type string: {:?}",
                arg.get("file_path")
            )
        })?
        .into();

    let content = arg
        .get("content")
        .ok_or(anyhow!("Expected object to contain `content`: {:?}", arg))?
        .str()
        .ok_or_else(|| {
            anyhow!(
                "Expected `content` to be of type string: {:?}",
                arg.get("content")
            )
        })?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .await?;

    file.write_all(content.as_bytes()).await?;
    let module = event
        .args()
        .and_then(|o| o.obj())
        .and_then(|o| o.get("module").cloned())
        .and_then(|v| v.str())
        .unwrap_or_default();
    core.throw_event(Event::new(format!("fsa_write_{}", module), None))
        .await;

    Ok(())
}

async fn fsa_read(event: &Event, core: &Box<dyn Core>) -> Result<()> {
    let arg = event
        .args()
        .ok_or(anyhow!("Expected argument, got nothing"))?
        .obj()
        .ok_or(anyhow!(
            "Expected argument to be of type Object, but got: {:?}",
            event.args()
        ))?;

    let file_path: PathBuf = arg
        .get("file_path")
        .ok_or(anyhow!("Expected object to contain `file_path`: {:?}", arg))?
        .str()
        .ok_or_else(|| {
            anyhow!(
                "Expected `file_path` to be of type string: {:?}",
                arg.get("file_path")
            )
        })?
        .into();

    let mut file = File::open(&file_path).await?;
    let mut buff = String::new();
    file.read_to_string(&mut buff).await?;

    let module = event
        .args()
        .and_then(|o| o.obj())
        .and_then(|o| o.get("module").cloned())
        .and_then(|v| v.str())
        .unwrap_or_default();

    core.throw_event(Event::new(
        format!("fsa_read_{}", module),
        Some(Value::new_obj().add("content", Value::Str(buff)).add(
            "file_path",
            Value::Str(file_path.to_str().unwrap_or_default().to_string()),
        )),
    ))
    .await;

    Ok(())
}

async fn fsa_dir(event: &Event, core: &Box<dyn Core>) -> Result<()> {
    let arg = event
        .args()
        .ok_or(anyhow!("Expected argument, got nothing"))?
        .obj()
        .or_else(|| {
            event.args().unwrap().str().map(|s| {
                let mut map = HashMap::new();
                map.insert("file_path".to_string(), Value::Str(s));
                map
            })
        })
        .ok_or(anyhow!(
            "Expected argument to be of type Object or Str, but got: {:?}",
            event.args()
        ))?;

    let file_path: PathBuf = arg
        .get("file_path")
        .ok_or(anyhow!("Expected object to contain `file_path`: {:?}", arg))?
        .str()
        .ok_or_else(|| {
            anyhow!(
                "Expected `file_path` to be of type string: {:?}",
                arg.get("file_path")
            )
        })?
        .into();

    let module = event
        .args()
        .and_then(|o| o.obj())
        .and_then(|o| o.get("module").cloned())
        .and_then(|v| v.str())
        .unwrap_or_default();

    let depth = event
        .args()
        .and_then(|o| o.obj())
        .and_then(|o| o.get("depth").cloned())
        .and_then(|v| v.int())
        .map(|v| v.abs() as usize);

    let ignore_hidden = event
        .args()
        .and_then(|o| o.obj())
        .and_then(|o| o.get("ignore_hidden").cloned())
        .and_then(|v| v.bool());

    core.throw_event(Event::new(
        format!("fsa_dir_{}", module),
        Some(objectify(
            walk_dir(file_path, FOptions::new(depth, None, ignore_hidden))?.unwrap(),
        )),
    ))
    .await;

    Ok(())
}

fn objectify(fo: Fo) -> Value {
    match fo {
        Fo::File(f) => {
            Value::new_obj().obj_add("file", Value::new_obj().obj_add("path", Value::Str(f)))
        }
        Fo::Folder(f, fos) => Value::new_obj().obj_add(
            "folder",
            Value::new_obj().obj_add("path", Value::Str(f)).obj_add(
                "contents",
                Value::List(fos.into_iter().map(|o| objectify(o)).collect()),
            ),
        ),
    }
}
