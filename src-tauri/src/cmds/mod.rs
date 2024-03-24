use crate::{
    errors::{set_lvl, ErrorLevel, NmideError},
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
pub async fn get_workspace(path: &str) -> Result<FolderOrFile, NmideError> {
    info!("Locking workspace");
    let mut ws = WORKSPACE.lock().await;

    info!("Init-ing on path: `{path:?}`");
    *ws = Workspace::init(Path::new(path)).map_err(|err| set_lvl(err, ErrorLevel::High))?;

    let res = FolderOrFile::Folder(
        ws.to_folder()
            .map_err(|err| set_lvl(err, ErrorLevel::High))?,
    );

    debug!("Response: `{res:?}`");

    Ok(res)
}

/// Saves the given content to the given file
pub fn save_file(path: &str, content: &str) -> Result<(), String> {
    todo!()
}

/// Closes the given file, without saving
pub fn close_file(path: &str) -> Result<(), String> {
    todo!()
}
