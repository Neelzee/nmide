use crate::explorer::explorer;
use anyhow::Result;
use nmide_std_lib::{
    attr::Attr,
    html::Html,
    map::{value::Value, Map},
    msg::Msg,
};
use std::{fs, path::PathBuf};

mod explorer;

const PATH_KEY: &str = "explorer-path";
const OPEN_MSG: &str = "explorer-path-open";

#[no_mangle]
pub extern "Rust" fn init() -> Map {
    Map::new().insert(PATH_KEY, Value::String(String::new()))
}

#[no_mangle]
pub extern "Rust" fn view(model: Map) -> Html {
    Html::Div {
        kids: vec![
            Html::Button {
                kids: vec![Html::Text("Open Folder".to_string())],
                attrs: vec![Attr::OnClick(Msg::PluginMsg(
                    OPEN_MSG.to_string(),
                    Value::String("/home/nmf/Documents/nmide".to_string()),
                ))],
            },
            explorer(model),
        ],
        attrs: Vec::new(),
    }
}

#[no_mangle]
pub extern "Rust" fn update(msg: Msg, model: Map) -> Map {
    match msg {
        Msg::PluginMsg(open_msg, path) if open_msg == OPEN_MSG && path.is_string() => {
            match get_path(&path.to_string().unwrap()) {
                Ok(files) => model.insert(PATH_KEY, files),
                Err(err) => model.insert(PATH_KEY, Value::String(format!("{err:?}"))),
            }
        }

        _ => model,
    }
}

fn get_path(root: &str) -> Result<Value> {
    fn _get_path(path: PathBuf, depth: usize) -> Result<Value> {
        if depth == 0 {
            return Ok(Value::List(Vec::new()));
        }

        if path.is_file() {
            return Ok(Value::String(
                path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            ));
        }

        let mut list = Value::List(Vec::new());

        for entry in fs::read_dir(path)?.filter_map(|k| k.ok()) {
            if entry.path().is_dir() {
                let folder_content = _get_path(entry.path(), depth - 1)?;
                list = list.push(
                    folder_content.push_start(Value::String(
                        entry
                            .path()
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                    )),
                );
            } else {
                list = list.push(Value::String(
                    entry
                        .path()
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                ));
            }
        }

        return Ok(list);
    }

    return _get_path(PathBuf::from(root), 3);
}
