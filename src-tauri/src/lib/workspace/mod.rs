pub mod ws_file;
pub mod ws_folder;

use crate::{
    lib::{
        either::Either,
        errors::NmideError,
        osops::get_paths,
        types::modules::{self, FolderOrFile},
        utils::funcs::os_to_str,
        workspace::{ws_file::WSFile, ws_folder::WSFolder},
    },
    nmrep,
};

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
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

        get_paths(path, i)
            .map(|val| {
                val.into_iter()
                    .map(|p| -> Either<_, _> {
                        if p.is_dir() {
                            Either::Left(WSFolder::new(p.as_path(), i - 1))
                        } else {
                            Either::Right(WSFile::new(&p))
                        }
                    })
                    .map(|b| b.transpose())
                    .fold(
                        NmideError {
                            val: Vec::new(),
                            rep: None,
                        },
                        |mut acc, e| {
                            acc.val.push(e.val);
                            if let Some(rep) = e.rep {
                                acc.push_nmide(rep)
                            } else {
                                acc
                            }
                        },
                    )
            })
            .vmap(|files| Workspace {
                root: path.to_owned(),
                files,
            })
    }

    pub fn to_folder(&self) -> NmideError<modules::Folder> {
        let (name, name_rep) =
            os_to_str(self.root.file_name().unwrap_or_default()).unwrap_with_err();

        let res = self
            .files
            .clone()
            .into_iter()
            .map(|e| match e {
                Either::Left(f) => Either::Left(f.to_folder()),
                Either::Right(f) => Either::Right(f.to_file()),
            })
            .map(|e| e.transpose())
            .fold(NmideError::new(Vec::new()), |mut err, e| {
                err.val.push(e.val);
                if let Some(rep) = e.rep {
                    err.push_nmide(rep)
                } else {
                    err
                }
            })
            .vmap(|e| -> Vec<FolderOrFile> { e.into_iter().map(|f| f.into()).collect::<Vec<_>>() })
            .vmap(|content| modules::Folder {
                name,
                path: self.root.to_str().unwrap_or_default().to_string(),
                content,
            });

        if let Some(rep) = name_rep {
            res.push_nmide(rep)
        } else {
            res
        }
    }
}
