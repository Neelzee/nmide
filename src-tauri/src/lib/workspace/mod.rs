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
    nmfold, nmrep,
};

use std::path::{Path, PathBuf};

use super::osops::get_folder_or_file;

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

        get_folder_or_file(path, i)
            .vmap(|val| -> Vec<_> {
                val.into_iter()
                    .map(|f| match f {
                        Either::Left(f) => Either::Left(f.to_wsfolder()),
                        Either::Right(f) => Either::Right(f.to_wsfile()),
                    })
                    .map(|e| e.transpose())
                    .collect()
            })
            .map(|e| nmfold!(e))
            .vmap(|files| Self {
                root: path.to_path_buf(),
                files,
            })
    }

    pub fn to_folder(&self) -> NmideError<modules::Folder> {
        self.files
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
                name: self.root.file_name().unwrap_or_default().to_os_string(),
                path: self.root.as_os_str().to_os_string(),
                content,
            })
    }
}
