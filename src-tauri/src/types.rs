use std::{
    io::{self, Read},
    path::Path,
};

use eyre::{Context, Error, Result};
use serde::{Deserialize, Serialize};

use crate::{osops::get_files, utils::funcs::os_to_str};

#[derive(Debug, Deserialize, Serialize)]
pub enum Fof {
    File(File),
    Folder(Folder),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "file")]
pub struct File {
    name: String,
    extension: String,
    path: String,
    content: String,
}

impl File {
    pub fn new(path: &Path) -> Result<File> {
        if !path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path is not a file",
            ))
            .wrap_err("Failed");
        }
        let name = os_to_str(
            path.file_name()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Faild getting filename",
                ))
                .wrap_err("failed")?,
        )?;

        let extension = os_to_str(
            path.extension()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed getting exsention",
                ))
                .wrap_err("failed")?,
        )?;

        let path_str = path
            .to_str()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed converting from Path to String",
            ))
            .wrap_err("failed")?
            .to_string();

        let mut content = String::new();

        let mut file = std::fs::File::open(path)?;

        file.read_to_string(&mut content)?;

        Ok(File {
            name,
            extension,
            path: path_str,
            content,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "folder")]
pub struct Folder {
    name: String,
    path: String,
    content: Vec<Fof>,
}

impl Folder {
    pub fn new(path: &Path) -> Result<Folder> {
        if !path.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path is not directory",
            ))
            .wrap_err("Failed");
        }

        let name = os_to_str(
            path.file_name()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Faild getting filename",
                ))
                .wrap_err("failed")?,
        )?;

        let extension = os_to_str(
            path.extension()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed getting exsention",
                ))
                .wrap_err("failed")?,
        )?;

        let path_str = path
            .to_str()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed converting from Path to String",
            ))
            .wrap_err("failed")?
            .to_string();

        let content = vec![get_files(path_str.clone())?];

        Ok(Folder {
            name,
            path: path_str,
            content,
        })
    }
}
