use crate::{errors::NmideError, nmrep, types, utils::funcs::os_to_str};
use either::Either;
use eyre::{Context, Result};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct WSFile {
    path: PathBuf,
    name: String,
    ext: String,
    is_opened: bool,
    content: Option<String>,
    file: Box<File>,
}

impl WSFile {
    pub fn new(path: &PathBuf, file: Box<File>) -> NmideError<WSFile> {
        let mut err = NmideError::empty();

        let (name, name_rep) = os_to_str((*path).file_name().unwrap_or_default()).unwrap_with_err();
        let (ext, ext_rep) = os_to_str(path.extension().unwrap_or_default()).unwrap_with_err();

        err.rep = nmrep!(name_rep, ext_rep);

        err.val = Some(WSFile {
            path: path.clone(),
            name: name.unwrap_or_default(),
            ext: ext.unwrap_or_default(),
            is_opened: false,
            content: None,
            file,
        });

        err
    }

    pub fn open(&mut self) -> Result<()> {
        let mut file = File::open(&(*self.path))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        self.content = Some(buf);

        Ok(())
    }

    pub fn save(&mut self) -> Result<()> {
        let mut writer = BufWriter::new((*self.file).try_clone().wrap_err("Failed")?);
        self.content.clone().and_then(|c| {
            writer
                .write_all(c.as_bytes())
                .wrap_err("Failed saving file")
                .ok()
        });
        Ok(())
    }

    pub fn close(&mut self) {
        self.content = None;
        self.is_opened = false;
    }

    pub fn to_file(&self) -> NmideError<types::File> {
        let mut err = NmideError::empty();

        let (path, path_rep) = os_to_str(self.path.clone().as_os_str()).unwrap_with_err();

        err.rep = path_rep;

        err.val = Some(types::File {
            name: self.name.clone(),
            extension: self.ext.clone(),
            path: path.unwrap_or_default(),
            content: self.content,
        });

        err
    }
}
