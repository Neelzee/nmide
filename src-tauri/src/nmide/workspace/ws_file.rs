use crate::{
    nmide::{errors::NmideError, types, utils::funcs::os_to_str},
    nmrep,
};
use eyre::Result;
use std::{ffi::OsString, path::PathBuf};
use tokio::io::BufReader;
use tokio::{fs::File, io::AsyncBufReadExt};

#[derive(Debug)]
pub struct WSFile {
    pub path: OsString,
    name: OsString,
    ext: OsString,
    content: Vec<String>,
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
            content: Vec::new(),
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
                content: Vec::new(),
                file: None,
            },
            rep: None,
        }
    }

    pub async fn open(&mut self) -> Result<Vec<String>> {
        let file = File::open(self.path.clone()).await?;

        let mut buffer = Vec::new();

        let mut reader = BufReader::new(file);

        let mut buf = String::new();

        while let Ok(i) = reader.read_line(&mut buf).await {
            // EOF
            if i == 0 {
                break;
            }
            buffer.push(buf.clone());
            buf.clear();
        }

        Ok(buffer)
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
