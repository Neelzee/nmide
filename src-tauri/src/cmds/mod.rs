use crate::types::FolderOrFile;
use eyre::{Context, Result};

/// Gets workspace
///
/// Should only be called once, as it also initializes a workspace
pub fn get_workspace(path: String) -> Result<FolderOrFile> {
    todo!()
}

/// Saves the given content to the given file
pub fn save_file(path: String, content: String) -> Result<()> {
    todo!()
}

/// Closes the given file, without saving
pub fn close_file(path: String) -> Result<()> {
    todo!()
}
