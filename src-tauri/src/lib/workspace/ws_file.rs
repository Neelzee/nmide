use crate::{
    lib::{errors::NmideError, types, utils::funcs::os_to_str},
    nmrep,
};

use std::{fs::File, path::PathBuf};

#[derive(Debug)]
pub struct WSFile {
    path: PathBuf,
    name: String,
    ext: String,
    content: Option<String>,
    file: Option<Box<File>>,
}

impl WSFile {
    pub fn empty() -> Self {
        Self {
            path: PathBuf::new(),
            name: String::new(),
            ext: String::new(),
            content: None,
            file: None,
        }
    }

    pub fn new(path: &PathBuf) -> NmideError<WSFile> {
        let (name, name_rep) = os_to_str((*path).file_name().unwrap_or_default()).unwrap_with_err();
        let (ext, ext_rep) = os_to_str(path.extension().unwrap_or_default()).unwrap_with_err();

        NmideError {
            val: WSFile {
                path: path.clone(),
                name,
                ext,
                content: None,
                file: None,
            },
            rep: nmrep!(name_rep, ext_rep),
        }
    }

    pub fn to_file(&self) -> NmideError<types::modules::File> {
        os_to_str(self.path.clone().as_os_str()).map(|err| NmideError {
            val: types::modules::File {
                name: self.name.clone(),
                extension: self.ext.clone(),
                path: err.val,
            },
            rep: err.rep,
        })
    }
}
