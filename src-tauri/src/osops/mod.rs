use eyre::Result;

use crate::types::Fof;

#[tauri::command]
pub fn get_files(path: String) -> Result<Fof> {
    todo!()
}
