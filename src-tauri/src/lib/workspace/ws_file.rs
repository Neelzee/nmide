use crate::{
    lib::{errors::NmideError, types, utils::funcs::os_to_str},
    nmrep,
};
use std::{ffi::OsString, fs::File, path::PathBuf};

#[derive(Debug)]
pub struct WSFile {
    pub path: OsString,
    name: OsString,
    ext: OsString,
    content: Option<String>,
    file: Option<Box<File>>,
}

impl Clone for WSFile {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            name: self.name.clone(),
            ext: self.ext.clone(),
            content: self.content.clone(),
            file: None,
        }
    }
}

impl WSFile {
    pub fn empty() -> Self {
        Self {
            path: OsString::new(),
            name: OsString::new(),
            ext: OsString::new(),
            content: None,
            file: None,
        }
    }
    pub fn pretty_display(&self) -> String {
        self.to_file().val.pretty_display()
    }
    pub fn new(path: &PathBuf) -> NmideError<WSFile> {
        let name = path.clone().file_name().unwrap_or_default().to_os_string();

        let ext = path
            .extension()
            .and_then(|s| Some(s.to_os_string()))
            .unwrap_or_default();

        NmideError {
            val: WSFile {
                path: path.as_os_str().to_os_string(),
                name,
                ext,
                content: None,
                file: None,
            },
            rep: None,
        }
    }

    pub fn to_file(&self) -> NmideError<types::modules::File> {
        NmideError::new(types::modules::File {
            name: self.name.clone(),
            extension: self.ext.clone(),
            path: self.path.as_os_str().to_os_string(),
            symbol: String::new(),
        })
    }
}
