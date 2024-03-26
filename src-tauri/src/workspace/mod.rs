pub mod ws_file;
pub mod ws_folder;

use crate::{
    either::Either,
    errors::{collect, NmideError, NmideReport},
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
        (&self.files)
            .into_iter()
            .map(|(_, v)| match v {
                Either::Left(ws) => Either::Left(ws.to_file()),
                Either::Right(ws) => Either::Right(ws.to_folder()),
            })
            .map(|e| e.transpose().vmap(|e| -> FolderOrFile { e.into() }))
            .fold(
                NmideError {
                    val: Vec::new(),
                    rep: None,
                },
                |mut err, e| {
                    err.val.push(e.val);
                    if let Some(rep) = e.rep {
                        err.push_nmide(rep);
                    }
                    err
                },
            )
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

    pub fn init(path: &Path) -> NmideError<Self> {
        info!("Initializing workspace on `{path:?}`");
        let i = 2;
        info!("Walking `{i}` deep");
        let mut dirs: HashMap<String, Either<NmideError<WSFile>, NmideError<WSFolder>>> =
            HashMap::new();

        let (paths, path_rep) = get_paths(path, i).unwrap_with_err();

        for p in paths {
            debug!("Path: `{p:?}`");
            let key = p.to_str().unwrap_or_default().to_string();

            if dirs.contains_key(&key) {
                warn!("{key:?} is in dir, with value: `{:?}`", dirs.get(&key));
            }

            if p.is_dir() {
                dirs.insert(key, Either::Right(WSFolder::new(p.as_path(), i - 1)));
            } else {
                dirs.insert(key, Either::Left(WSFile::new(&p)));
            }
        }

        let content = dirs
            .into_iter()
            .map(|(k, v)| (k, v.transpose().val))
            .collect::<HashMap<_, _>>();

        let content_rep = collect(dirs.into_iter().map(|(_, v)| v.transpose()).collect()).1;

        NmideError {
            val: Workspace {
                root: path.to_owned(),
                files: content,
            },
            rep: content_rep,
        }
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
