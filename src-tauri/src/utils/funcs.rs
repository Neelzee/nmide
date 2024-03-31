use std::ffi::OsStr;

use crate::{
    errors::{ErrorLevel, NmideError, NmideReport},
    types::FolderOrFile,
};

pub fn os_to_str(s: &OsStr) -> NmideError<String> {
    NmideError {
        val: s
            .to_str()
            .and_then(|s| Some(s.to_string()))
            .unwrap_or(format!("{s:?}")),
        rep: Some(NmideReport {
            msg: format!("Failed converting String: `{s:?}`"),
            lvl: ErrorLevel::Low,
            tag: Vec::new(),
            stack: Vec::new(),
            origin: "os_to_str".to_string(),
        }),
    }
}

pub fn pretty_display(files: &Vec<FolderOrFile>, lvl: usize) -> String {
    let mut s = String::new();

    for f in files {
        match f {
            FolderOrFile::File(f) => s += &format!("{}{}\n", " ".repeat(lvl), f.name),
            FolderOrFile::Folder(f) => {
                s += &format!("{}{}\n", " ".repeat(lvl), f.name);
                s += &pretty_display(&f.content, lvl + 2);
            }
        }
    }

    s
}
