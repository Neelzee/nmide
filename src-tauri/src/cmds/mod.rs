use crate::{
    errors::{ErrorLevel, NmideError, NmideReport},
    nmrep,
    types::modules::{File, FolderOrFile},
    utils::funcs::pretty_display,
    workspace::Workspace,
    WORKSPACE,
};
use log::{debug, info};
use std::path::Path;

/// Gets workspace
///
/// Should only be called once, as it also initializes a workspace
#[tauri::command]
pub async fn get_workspace(path: &str) -> Result<NmideError<FolderOrFile>, NmideReport> {
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

    Ok(res.vmap(|f| {
        let r = FolderOrFile::Folder(f);
        //debug!("{}", pretty_display(&vec![r.clone()], 0));
        r
    }))
}

/// Saves the given content to the given file
pub fn save_file(path: &str, content: &str) -> Result<(), String> {
    todo!()
}

/// Closes the given file, without saving
pub fn close_file(path: &str) -> Result<(), String> {
    todo!()
}
