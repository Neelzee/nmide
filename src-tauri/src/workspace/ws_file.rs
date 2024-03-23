use crate::{errors::NmideError, types, utils::funcs::os_to_str};
use either::Either;
use eyre::{eyre, Context, OptionExt, Result};
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

#[derive(Debug)]
pub struct WSFolder {
    path: PathBuf,
    name: String,
    content: Vec<Either<WSFile, WSFolder>>,
}

impl WSFile {
    pub fn new(path: &PathBuf, file: Box<File>) -> Result<WSFile> {
        Ok(WSFile {
            path: path.clone(),
            name: (*path)
                .file_name()
                .and_then(|op| Some(os_to_str(op)))
                .unwrap()?,
            ext: path
                .extension()
                .and_then(|op| os_to_str(op).ok())
                .unwrap_or(String::new()),
            is_opened: false,
            content: None,
            file,
        })
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

    pub fn to_file(&self) -> Result<types::File> {
        Ok(types::File {
            name: self.name.clone(),
            extension: self.ext.clone(),
            path: os_to_str(self.path.clone().as_os_str())?,
            content: {
                Some(
                    match &self.content {
                        Some(f) => Ok::<_, eyre::ErrReport>(f.clone()),
                        None => {
                            let mut buffer = String::new();
                            let mut reader = BufReader::new(File::open(self.path.clone()).wrap_err(
                            "Failed opening file for reading, when converting from WSFile to File",
                        )?);
                            reader.read_to_string(&mut buffer).wrap_err(
                            "Failed reading content from file, when converting from WSFile to File",
                        )?;

                            Ok(buffer)
                        }
                    }
                    .wrap_err("Failed reading content from file")?,
                )
            },
        })
    }
}
