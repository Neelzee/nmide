pub mod ws;

use crate::{
    osops::get_paths,
    types,
    utils::funcs::os_to_str,
    workspace::ws::{ws_file::WSFile, ws_folder::WSFolder},
};
use either::Either;
use eyre::{Context, OptionExt, Result};
#[warn(unused_imports)]
use log::{debug, info, warn};
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Workspace {
    root: PathBuf,
    files: HashMap<String, Either<WSFile, WSFolder>>,
}

impl Workspace {
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
        let mut dirs: HashMap<String, Either<WSFile, WSFolder>> = HashMap::new();
        for p in get_paths(path, 1)? {
            debug!("Path: `{p:?}`");
            let r = p
                .to_str()
                .ok_or_eyre("Failed converting into valid UTF-8 String: `{p:?}`");
            let key: String;
            if r.is_err() {
                warn!("Error: `{r:?}`");
                key = format!("{p:?}")
            } else {
                key = r.unwrap().to_string()
            }
            if !p.is_dir() {
                dirs.insert(
                    key,
                    Either::Left(WSFile::new(
                        &p,
                        Box::new(
                            File::open(&p).wrap_err("Failed opening file for WSFile creation")?,
                        ),
                    )?),
                );
            } else {
                dirs.insert(key, Either::Right(WSFolder::new(p.as_path(), 0)?));
            }
        }

        Ok(Self {
            root: path.to_owned(),
            files: dirs,
        })
    }

    pub fn cheap_files_clone(&self) -> HashMap<String, types::FolderOrFile> {
        for (k, v) in &self.files {
            todo!()
        }
        todo!()
    }

    pub fn to_folder(&self) -> Result<types::Folder> {
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
            content: todo!(),
        })
    }
}
