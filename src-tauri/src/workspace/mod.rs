pub mod ws_file;
pub mod ws_folder;

use crate::{
    either::Either,
    errors::{NmideError, NmideReport},
    osops::{get_folder_or_file, get_paths},
    types::{self, Folder, FolderOrFile},
    utils::funcs::os_to_str,
    workspace::{ws_file::WSFile, ws_folder::WSFolder},
};
use eyre::{Context, OptionExt, Result};
#[warn(unused_imports)]
use log::{debug, info, warn};
use std::{
    collections::HashMap,
    fmt::write,
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Workspace {
    root: PathBuf,
    files: HashMap<String, Either<WSFile, WSFolder>>,
}

impl Workspace {
    fn copy_files(&self) -> NmideError<Vec<FolderOrFile>> {
        let mut vec = Vec::new();
        for (_, v) in &self.files {
            match v {
                Either::Left(f) => vec.push(FolderOrFile::File(f.to_file()?)),
                Either::Right(f) => vec.push(FolderOrFile::Folder(f.to_folder()?)),
            }
        }

        Ok(vec)
    }

    pub fn empty() -> Self {
        Self {
            root: PathBuf::new(),
            files: HashMap::new(),
        }
    }

    pub fn new(root: &Path, files: HashMap<String, Either<WSFile, WSFolder>>) -> Workspace {
        Workspace {
            root: root.to_owned(),
            files,
        }
    }

    pub fn init(path: &Path) -> Result<Self> {
        info!("Initializing workspace on `{path:?}`");
        let i = 2;
        info!("Walking `{i}` deep");
        let mut dirs: HashMap<String, Either<WSFile, WSFolder>> = HashMap::new();
        for p in get_paths(path, i)? {
            debug!("Path: `{p:?}`");
            let r = p
                .to_str()
                .ok_or_eyre("Failed converting into valid UTF-8 String: `{p:?}`");
            let key = if r.is_err() {
                warn!("Error: `{r:?}`");
                format!("{p:?}")
            } else {
                r.unwrap().to_string()
            };
            if p.is_dir() {
                dirs.insert(key, Either::Right(WSFolder::new(p.as_path(), i - 1)?));
            } else {
                dirs.insert(key, Either::Left(WSFile::new(&p)?));
            }
        }

        Ok(Self {
            root: path.to_owned(),
            files: dirs,
        })
    }

    pub fn to_folder(&self) -> NmideError<types::Folder> {
        Ok(types::Folder {
            name: os_to_str(self.root.as_path().file_name().ok_or_eyre(format!(
                "Failed getting file name from: `{:?}` to UTF-8 String",
                self.root
            ))?)?,
            path: self
                .root
                .to_str()
                .ok_or_eyre(format!(
                    "Failed converting root: `{:?}` to UTF-8 String",
                    self.root
                ))?
                .to_string(),
            content: self.copy_files()?,
        })
    }
}
