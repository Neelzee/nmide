use std::{
    io::{self, Read},
    path::Path,
};

use crate::utils::funcs::os_to_str;
use either::Either;
use eyre::{Context, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum FolderOrFile {
    File(File),
    Folder(Folder),
}

impl Into<Either<File, Folder>> for FolderOrFile {
    fn into(self) -> Either<File, Folder> {
        match self {
            FolderOrFile::File(f) => Either::Left(f),
            FolderOrFile::Folder(f) => Either::Right(f),
        }
    }
}

impl Into<FolderOrFile> for Either<File, Folder> {
    fn into(self) -> FolderOrFile {
        match self {
            Either::Left(l) => FolderOrFile::File(l),
            Either::Right(r) => FolderOrFile::Folder(r),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub extension: String,
    pub path: String,
    pub content: Option<String>,
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
                    "Failed getting filename",
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
            content: Some(content),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Folder {
    pub name: String,
    pub path: String,
    pub content: Vec<FolderOrFile>,
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
                    "Failed getting filename",
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

        let content = vec![];

        Ok(Folder {
            name,
            path: path_str,
            content,
        })
    }
}
