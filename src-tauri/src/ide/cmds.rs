use crate::nmide::{
    errors::{NmideError, NmideReport},
    workspace::Workspace,
    WORKSPACE,
};
use serde_json::Value;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[tauri::command]
/// Gets workspace
///
/// Should only be called once, as it also initializes a workspace
pub async fn get_workspace(path: &str) -> Result<NmideError<Value>, NmideReport> {
    if path.is_empty() {
        return Err(NmideReport::new("Can't open empty path", "get_workspace"));
    }

    let mut ws = WORKSPACE.lock().await;

    let (new_ws, rep) = Workspace::init(Path::new(path)).unwrap_with_err();

    *ws = new_ws;

    let mut res = ws.to_folder();

    if let Some(r) = rep {
        res = res.push_nmide(r);
    }

    match serde_json::to_value(&res.val) {
        Ok(val) => Ok(NmideError { val, rep: res.rep }),
        Err(err) => Err(NmideReport::new(
            format!("{err:?}"),
            "get_workspace".to_string(),
        )),
    }
}

#[tauri::command]
/// Gets the content in the given file
pub async fn get_content(path: &str) -> Result<NmideError<Vec<String>>, NmideReport> {
    if path.is_empty() {
        return Err(NmideReport::new("Can't open empty path", "get_content"));
    }

    if PathBuf::from(path).is_dir() {
        return Err(NmideReport::new(
            format!("Can't get file content from a directory: `{path}`"),
            "get_content".to_string(),
        ));
    }

    let mut ws = WORKSPACE.lock().await;

    let content = ws.open_file(OsString::from(path)).await;

    Ok(content)
}

/// Saves the given content to the given file
pub fn save_file(_path: &str, _content: &str) -> Result<(), String> {
    todo!()
}

/// Closes the given file, without saving
pub fn close_file(_path: &str) -> Result<(), String> {
    todo!()
}
