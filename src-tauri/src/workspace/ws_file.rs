use std::{fs::File, io::Read, path::Path};

use eyre::{eyre, Context, Result};

use crate::errors::NmideError;

#[derive(Debug)]
pub struct WSFile {
    path: Box<Path>,
    name: String,
    ext: String,
    is_opened: bool,
    content: Option<String>,
    file: Box<File>,
}

impl WSFile {
    pub fn new(path: &Path) -> Result<Self> {
        Ok(WSFile {
            path: path.into(),
            name: path
                .file_name()
                .ok_or(eyre!(NmideError::OptionToResult("OsStr".to_string())))
                .wrap_err(format!("Failed getting file name from path: `{:?}`", path))?
                .to_str()
                .ok_or(eyre!(NmideError::OptionToResult("OsStr".to_string())))
                .wrap_err(format!(
                    "Failed getting converting to String from path: `{:?}`",
                    path
                ))?
                .to_string(),
            ext: path
                .extension()
                .ok_or(eyre!(NmideError::OptionToResult("OsStr".to_string())))
                .wrap_err(format!("Failed getting exstension from path: {:?}", path))?
                .to_str()
                .ok_or(eyre!(NmideError::OptionToResult("OsStr".to_string())))
                .wrap_err(format!(
                    "Failed getting converting to String from path: `{:?}`",
                    path
                ))?
                .to_string(),
            is_opened: false,
            content: None,
            file: Box::new(
                File::open(path).wrap_err(format!("Failed opening file at path: {:?}", path))?,
            ),
        })
    }

    pub fn open(&mut self) -> Result<()> {
        let mut file = File::open(self.path.clone())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        self.content = Some(buf);

        Ok(())
    }

    pub fn close(&mut self) {
        self.content = None;
        self.is_opened = false;
    }
}
