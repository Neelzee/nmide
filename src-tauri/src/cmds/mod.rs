use eyre::Context;

use crate::{
    errors::{set_lvl, ErrorLevel, NmideError},
    types::FolderOrFile,
    workspace::Workspace,
    WORKSPACE,
};
use std::path::Path;

/// Gets workspace
///
/// Should only be called once, as it also initializes a workspace
#[tauri::command]
pub async fn get_workspace(path: &str) -> Result<FolderOrFile, NmideError> {
    let mut ws = WORKSPACE
        .try_lock()
        .wrap_err("Failed locking workspace")
        .map_err(|err| set_lvl(err, ErrorLevel::High))?;

    *ws = Workspace::init(Path::new(path)).map_err(|err| set_lvl(err, ErrorLevel::High))?;

    Ok(FolderOrFile::Folder(
        ws.to_folder()
            .map_err(|err| set_lvl(err, ErrorLevel::High))?,
    ))
}

/// Saves the given content to the given file
pub fn save_file(path: &str, content: &str) -> Result<(), String> {
    todo!()
}

/// Closes the given file, without saving
pub fn close_file(path: &str) -> Result<(), String> {
    todo!()
}
