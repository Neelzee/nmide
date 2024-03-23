use crate::{types::FolderOrFile, workspace::Workspace, WORKSPACE};
use eyre::{Context, Result};
use std::path::Path;

/// Gets workspace
///
/// Should only be called once, as it also initializes a workspace
pub async fn get_workspace(path: String) -> Result<FolderOrFile> {
    let mut ws = WORKSPACE
        .try_lock()
        .wrap_err(format!("Failed init. workspace with path: `{path:?}`"))?;

    *ws = Workspace::init(Path::new(&path))?;

    return Ok(FolderOrFile::Folder(ws.to_folder()?));
}

/// Saves the given content to the given file
pub fn save_file(path: String, content: String) -> Result<()> {
    todo!()
}

/// Closes the given file, without saving
pub fn close_file(path: String) -> Result<()> {
    todo!()
}
