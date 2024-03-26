use crate::{
    either::Either,
    errors::{ErrorLevel, NmideError, NmideReport},
    nmrep,
    utils::funcs::os_to_str,
    workspace::{ws_file::WSFile, ws_folder::WSFolder},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub enum FolderOrFile {
    File(File),
    Folder(Folder),
}

impl From<Either<File, Folder>> for FolderOrFile {
    fn from(value: Either<File, Folder>) -> Self {
        match value {
            Either::Left(f) => FolderOrFile::File(f),
            Either::Right(f) => FolderOrFile::Folder(f),
        }
    }
}

impl From<FolderOrFile> for Either<File, Folder> {
    fn from(value: FolderOrFile) -> Self {
        match value {
            FolderOrFile::File(f) => Either::Left(f),
            FolderOrFile::Folder(f) => Either::Right(f),
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
    /// Creates a File instance
    ///
    /// Will not read the contents of the File
    pub fn new(path: &Path) -> NmideError<File> {
        let mut err = NmideError::empty();

        if !path.is_file() {
            err.rep = Some(NmideReport {
                msg: format!("Path: `{path:?}` is not a file"),
                lvl: ErrorLevel::Low,
                origin: "File::new".to_string(),
                tag: Vec::new(),
                stack: Vec::new(),
            });
            return err;
        }
        let (name, name_rep) = os_to_str(path.file_name().unwrap_or_default()).unwrap_with_err();

        let (extension, extension_rep) =
            os_to_str(path.extension().unwrap_or_default()).unwrap_with_err();

        let (path_str, path_str_rep) = NmideError {
            val: path.to_str().and_then(|s| Some(s.to_string())),
            rep: Some(NmideReport {
                msg: format!("Failed converting Path to String: `{path:?}`"),
                lvl: ErrorLevel::Low,
                tag: Vec::new(),
                stack: Vec::new(),
                origin: "Folder::new".to_string(),
            }),
        }
        .unwrap_with_err();

        err.rep = nmrep!(name_rep, extension_rep, path_str_rep);

        err.val = Some(File {
            name: name.unwrap_or_default(),
            extension: extension.unwrap_or_default(),
            path: path_str.unwrap_or_default(),
            content: Some(String::new()),
        });

        err
    }

    pub fn to_wsfile(&self) -> NmideError<WSFile> {
        let file = NmideError {
            val: fs::File::open(self.path).ok(),
            rep: Some(NmideReport {
                msg: format!("Failed creating File from path: `{:?}`", self.path),
                lvl: ErrorLevel::Medium,
                tag: Vec::new(),
                stack: Vec::new(),
                origin: "types::File::to_wsfile".to_string(),
            }),
        };

        if let Some(err) = file.or_else(|f| WSFile::new(&PathBuf::from(&self.path), Box::new(f))) {
            return err;
        } else {
            let mut err = NmideError::empty();
            err.rep = file.rep;
            return err;
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Folder {
    pub name: String,
    pub path: String,
    pub content: Vec<FolderOrFile>,
}

impl Folder {
    /// Creates a Folder instance
    ///
    /// Will not go any level deeper, ie. content will always be empty
    pub fn new(path: &Path) -> NmideError<Folder> {
        let mut err = NmideError {
            val: None,
            rep: None,
        };

        if !path.is_dir() {
            err.rep = Some(NmideReport {
                msg: format!("The given path: `{path:?}` is not a directory"),
                lvl: ErrorLevel::High,
                tag: Vec::new(),
                stack: Vec::new(),
                origin: "Folder::new".to_string(),
            });
            return err;
        }

        let (name, name_rep) = os_to_str(path.file_name().unwrap_or_default()).unwrap_with_err();

        let (extension, extension_rep) =
            os_to_str(path.extension().unwrap_or_default()).unwrap_with_err();

        let (path_str, path_str_rep) = NmideError {
            val: path.to_str().and_then(|s| Some(s.to_string())),
            rep: Some(NmideReport {
                msg: format!("Failed converting Path to String: `{path:?}`"),
                lvl: ErrorLevel::Low,
                tag: Vec::new(),
                stack: Vec::new(),
                origin: "Folder::new".to_string(),
            }),
        }
        .unwrap_with_err();

        err.val = Some(Folder {
            name: name.unwrap_or_default(),
            path: path_str.unwrap_or_default(),
            content: Vec::new(),
        });

        err
    }
    /// Creates a WSFolder, with path 1
    pub fn to_wsfolder(self) -> NmideError<WSFolder> {
        let mut err = NmideError::empty();

        let ws = WSFolder::new(&Path::new(&self.path), 0)
            .val
            .and_then(|mut w| {
                w.push_content(
                    self.content
                        .into_iter()
                        .map(|f| -> Either<NmideError<WSFile>, NmideError<WSFolder>> {
                            match f {
                                FolderOrFile::File(f) => Either::Left(f.to_wsfile()),
                                FolderOrFile::Folder(f) => Either::Right(f.to_wsfolder()),
                            }
                        })
                        .collect(),
                );
                Some(w)
            });

        err
    }
}
