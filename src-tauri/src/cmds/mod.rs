use crate::{types::FolderOrFile, workspace::Workspace, WORKSPACE};
use std::path::Path;

/// Gets workspace
///
/// Should only be called once, as it also initializes a workspace
#[tauri::command]
#[allow(clippy::used_underscore_binding)]
pub async fn get_workspace(path: &str) -> Result<FolderOrFile, String> {
    if let Ok(mut ws) = WORKSPACE.try_lock() {
        if let Ok(newws) = Workspace::init(Path::new(&path)) {
            *ws = newws;
        } else {
            return Err(format!("Failed init. workspace with path: `{path:?}`"));
        }

        if let Ok(fof) = ws.to_folder() {
            return Ok(FolderOrFile::Folder(fof));
        }
        return Err(format!(
            "Failed converting workspace to fof with path: `{path:?}`"
        ));
    }
    Err(format!("Failed locking ws with path: `{path:?}`"))
}

/// Saves the given content to the given file
pub fn save_file(path: &str, content: &str) -> Result<(), String> {
    todo!()
}

/// Closes the given file, without saving
pub fn close_file(path: &str) -> Result<(), String> {
    todo!()
}
