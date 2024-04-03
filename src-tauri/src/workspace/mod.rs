pub mod ws_file;
pub mod ws_folder;

use crate::{
    either::Either,
    errors::NmideError,
    nmrep,
    osops::get_paths,
    types::modules::{self, FolderOrFile},
    utils::funcs::os_to_str,
    workspace::{ws_file::WSFile, ws_folder::WSFolder},
};

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Workspace {
    root: PathBuf,
    files: Vec<Either<WSFolder, WSFile>>,
}

impl Workspace {
    pub fn len(&self) -> usize {
        self.files.len()
    }
    pub fn get_files(&self) -> &Vec<Either<WSFolder, WSFile>> {
        &self.files
    }

    fn copy_files(&self) -> NmideError<Vec<FolderOrFile>> {
        (&self.files)
            .into_iter()
            .map(|v| match v {
                Either::Left(ws) => Either::Left(ws.to_folder()),
                Either::Right(ws) => Either::Right(ws.to_file()),
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
            files: Vec::new(),
        }
    }

    pub fn new(root: &Path, files: Vec<Either<WSFolder, WSFile>>) -> Workspace {
        Workspace {
            root: root.to_owned(),
            files,
        }
    }

    pub fn init(path: &Path) -> NmideError<Self> {
        let i = 2;

        let (paths, path_rep) = get_paths(path, i).unwrap_with_err();

        let (files, files_rep) = paths
            .into_iter()
            .map(|p| -> Either<_, _> {
                if p.is_dir() {
                    Either::Left(WSFolder::new(p.as_path(), i - 1))
                } else {
                    Either::Right(WSFile::new(&p))
                }
            })
            .map(|b| -> NmideError<Either<_, _>> { b.transpose() })
            .fold(
                NmideError {
                    val: Vec::new(),
                    rep: None,
                },
                |mut acc, e| {
                    acc.val.push(e.val);
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

    pub fn to_folder(&self) -> NmideError<modules::Folder> {
        let (name, name_rep) =
            os_to_str(self.root.file_name().unwrap_or_default()).unwrap_with_err();

        let (content, content_rep) = self.copy_files().unwrap_with_err();

        NmideError {
            val: modules::Folder {
                name,
                path: self.root.to_str().unwrap_or_default().to_string(),
                content,
            },
            rep: nmrep!(name_rep, content_rep),
        }
    }
}
