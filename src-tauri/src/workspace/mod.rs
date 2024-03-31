pub mod ws_file;
pub mod ws_folder;

use crate::{
    either::Either,
    errors::{NmideError, NmideReport},
    nmrep,
    osops::{get_folder_or_file, get_paths},
    types::{self, Folder, FolderOrFile},
    utils::funcs::os_to_str,
    workspace::{ws_file::WSFile, ws_folder::WSFolder},
};
use eyre::{Context, OptionExt, Result};
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
    pub fn get_files(&self) -> Vec<&Either<WSFile, WSFolder>> {
        self.files.values().collect()
    }

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
                        err = err.push_nmide(rep);
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
        let i = 3;

        let (paths, path_rep) = get_paths(path, i).unwrap_with_err();

        let (files, files_rep) = paths
            .into_iter()
            .map(|p| -> (String, Either<_, _>) {
                let key = p.to_str().unwrap_or_default().to_string();

                if p.is_dir() {
                    (key, Either::Right(WSFolder::new(p.as_path(), i - 1)))
                } else {
                    (key, Either::Left(WSFile::new(&p)))
                }
            })
            .map(|(a, b)| -> (String, NmideError<Either<_, _>>) { (a, b.transpose()) })
            .fold(
                NmideError {
                    val: HashMap::new(),
                    rep: None,
                },
                |mut acc, (k, e)| {
                    acc.val.insert(k, e.val);
                    if let Some(rep) = e.rep {
                        acc = acc.push_nmide(rep);
                    }
                    acc
                },
            )
            .unwrap_with_err();

        NmideError {
            val: Workspace {
                root: path.to_owned(),
                files,
            },
            rep: nmrep!(path_rep, files_rep),
        }
    }

    pub fn to_folder(&self) -> NmideError<types::Folder> {
        let (name, name_rep) =
            os_to_str(self.root.file_name().unwrap_or_default()).unwrap_with_err();

        let (content, content_rep) = self.copy_files().unwrap_with_err();

        NmideError {
            val: types::Folder {
                name,
                path: self.root.to_str().unwrap_or_default().to_string(),
                content,
            },
            rep: nmrep!(name_rep, content_rep),
        }
    }
}
