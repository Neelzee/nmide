use crate::{types::FolderOrFile, workspace::Workspace, WORKSPACE};
use std::path::Path;

/// Gets workspace
///
/// Should only be called once, as it also initializes a workspace
#[tauri::command]
pub async fn get_workspace(path: String) -> Result<FolderOrFile, String> {
    if let Ok(mut ws) = WORKSPACE.try_lock() {
        if let Ok(new_ws) = Workspace::init(Path::new(&path)) {
            *ws = new_ws;
        } else {
            return Err(format!("Failed init. workspace with path: `{path:?}`"));
        }

        if let Ok(fof) = ws.to_folder() {
            return Ok(FolderOrFile::Folder(fof));
        } else {
            return Err(format!(
                "Failed converting workspace to fof with path: `{path:?}`"
            ));
        }
    } else {
        return Err(format!("Failed locking ws with path: `{path:?}`"));
    }
}

/// Saves the given content to the given file
pub fn save_file(path: String, content: String) -> Result<(), String> {
    todo!()
}

/// Closes the given file, without saving
pub fn close_file(path: String) -> Result<(), String> {
    todo!()
}
