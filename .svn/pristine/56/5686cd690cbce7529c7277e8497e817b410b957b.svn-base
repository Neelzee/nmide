use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::Path,
};

use eyre::{eyre, Context, Result};

use crate::{errors::NmideError, utils::funcs::os_to_str};

#[derive(Debug)]
pub struct WSFile<'a> {
    path: &'a Path,
    name: String,
    ext: String,
    is_opened: bool,
    content: Option<String>,
    file: &'a File,
}

impl WSFile<'_> {
    pub fn new<'a>(path: &'a Path, file: &'a File) -> Result<WSFile<'a>> {
        Ok(WSFile {
            path: path,
            name: path
                .file_name()
                .and_then(|op| os_to_str(op).ok())
                .ok_or(eyre!(NmideError::OptionToResult("OsStr".to_string())))?
                .to_string(),
            ext: path
                .extension()
                .and_then(|op| os_to_str(op).ok())
                .ok_or(eyre!(NmideError::OptionToResult("OsStr".to_string())))?,
            is_opened: false,
            content: None,
            file: file,
        })
    }

    pub fn open(&mut self) -> Result<()> {
        let mut file = File::open(self.path)?;
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
}
