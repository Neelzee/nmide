use crate::{
    errors::{ErrorLevel, NmideError},
    nmrep,
    types::FolderOrFile,
    workspace::Workspace,
    WORKSPACE,
};
use log::{debug, info};
use std::path::Path;

/// Gets workspace
///
/// Should only be called once, as it also initializes a workspace
#[tauri::command]
pub async fn get_workspace(path: &str) -> Result<NmideError<FolderOrFile>, ()> {
    info!("Locking workspace");
    let mut ws = WORKSPACE.lock().await;

    info!("Init-ing on path: `{path:?}`");
    let (new_ws, rep) = Workspace::init(Path::new(path)).unwrap_with_err();

    *ws = new_ws;

    let mut res = ws.to_folder();

    if let Some(r) = rep {
        res = res.push_nmide(r);
    }

    debug!("Response: `{res:?}`");

    Ok(res.vmap(|f| FolderOrFile::Folder(f)))
}

/// Saves the given content to the given file
pub fn save_file(path: &str, content: &str) -> Result<(), String> {
    todo!()
}

/// Closes the given file, without saving
pub fn close_file(path: &str) -> Result<(), String> {
    todo!()
}
