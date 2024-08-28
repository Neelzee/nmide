use crate::{editor::editor, error_log::error_log, explorer::explorer};
use anyhow::Result;
use nmide_std_lib::{
    attr::Attr,
    html::Html,
    map::{value::Value, Map},
    msg::Msg,
};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

mod editor;
mod explorer;
mod error_log {
    use nmide_std_lib::{attr::Attr, html::Html, map::Map};

    use crate::ERROR_KEY;

    pub(crate) fn error_log(model: &Map) -> Html {
        match model.lookup(ERROR_KEY) {
            Some(val) => Html::Div {
                kids: vec![Html::Text(format!("{val:?}"))],
                attrs: vec![Attr::Id("errors".to_string())],
            },
            None => Html::Div {
                kids: Vec::new(),
                attrs: vec![Attr::Id("errors".to_string())],
            },
        }
    }
}

pub const PATH_KEY: &str = "explorer-path";
pub const OPEN_MSG: &str = "explorer-path-open";
pub const OPEN_FILE_MSG: &str = "editor-open-file";
pub const FILE_CONTENT_KEY: &str = "editor-file-content";
pub const SAVE_FILE_MSG: &str = "editor-save-file";
pub const CLOSE_FILE_MSG: &str = "editor-close-file";
pub const ERROR_KEY: &str = "ide-error";

#[no_mangle]
pub extern "Rust" fn init() -> Map {
    Map::new().insert(PATH_KEY, Value::String(String::new()))
}

#[no_mangle]
pub extern "Rust" fn view(model: Map) -> Html {
    Html::Div {
        kids: vec![
            error_log(&model),
            Html::Button {
                kids: vec![Html::Text("Open Folder".to_string())],
                attrs: vec![Attr::OnClick(Msg::OpenFolderDialog(
                    OPEN_MSG.to_string(),
                    Value::String("/home/nmf/Documents/nmide".to_string()),
                ))],
            },
            editor(&model),
            explorer(&model),
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
                Err(err) => model.insert(ERROR_KEY, Value::String(format!("{err:?}"))),
            }
        }
        Msg::PluginMsg(open_file_msg, path)
            if open_file_msg == OPEN_FILE_MSG && path.is_string() =>
        {
            match get_file_content(&path.to_string().unwrap()) {
                Ok(v) => model
                    .insert(FILE_CONTENT_KEY, v)
                    .insert(OPEN_FILE_MSG, path)
                    .insert(CLOSE_FILE_MSG, Value::Bool(false)),
                Err(err) => model.insert(
                    ERROR_KEY,
                    Value::String(format!("{err:?} from path: {path:?}")),
                ),
            }
        }
        Msg::PluginMsg(close_file_msg, _) if close_file_msg == CLOSE_FILE_MSG => {
            model.insert(CLOSE_FILE_MSG, Value::Bool(true))
        }
        Msg::PluginMsg(save_file_msg, content)
            if save_file_msg == SAVE_FILE_MSG && content.is_string() =>
        {
            match model.lookup(OPEN_FILE_MSG) {
                Some(val) if val.is_string() => match write_file_content(
                    &val.to_string().unwrap(),
                    &content.to_string().unwrap(),
                ) {
                    Ok(_) => model,
                    Err(err) => model.insert(
                        ERROR_KEY,
                        Value::String(format!("Error: {err:?}, when saving file: {val:?}")),
                    ),
                },
                Some(val) => model.insert(
                    ERROR_KEY,
                    Value::String(format!("{val:?} is not string-type")),
                ),
                None => model.insert(
                    ERROR_KEY,
                    Value::String("Can't save, no open file".to_string()),
                ),
            }
        }
        _ => model,
    }
}

fn write_file_content(path: &str, content: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn get_file_content(path: &str) -> Result<Value> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(Value::String(buffer))
}

fn get_path(root: &str) -> Result<Value> {
    fn _get_path(path: PathBuf, depth: usize) -> Result<Value> {
        if depth == 0 {
            return Ok(Value::List(Vec::new()));
        }

        if path.is_file() {
            return Ok(Value::String(path.to_string_lossy().to_string()));
        }

        let mut list = Value::List(Vec::new());

        for entry in fs::read_dir(path)?.filter_map(|k| k.ok()) {
            if entry.path().is_dir() {
                let folder_content = _get_path(entry.path(), depth - 1)?;
                list = list.push(
                    folder_content
                        .push_start(Value::String(entry.path().to_string_lossy().to_string())),
                );
            } else {
                list = list.push(Value::String(entry.path().to_string_lossy().to_string()));
            }
        }

        return Ok(list);
    }

    return _get_path(PathBuf::from(root), 3);
}
