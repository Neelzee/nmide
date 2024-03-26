use crate::{
    either::Either,
    errors::{ErrorLevel, NmideError, NmideReport},
    nmrep,
    utils::funcs::os_to_str,
    workspace::{ws_file::WSFile, ws_folder::WSFolder},
};
use serde::{Deserialize, Serialize};
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
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            extension: String::new(),
            path: String::new(),
            content: None,
        }
    }

    /// Creates a File instance
    ///
    /// Will not read the contents of the File
    pub fn new(path: &Path) -> NmideError<File> {
        if !path.is_file() {
            return NmideError {
                val: File::empty(),
                rep: Some(NmideReport {
                    msg: format!("Path: `{path:?}` is not a file"),
                    lvl: ErrorLevel::Low,
                    origin: "File::new".to_string(),
                    tag: Vec::new(),
                    stack: Vec::new(),
                }),
            };
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

        NmideError {
            val: File {
                name,
                extension,
                path: path_str.unwrap_or_default(),
                content: Some(String::new()),
            },
            rep: nmrep!(name_rep, extension_rep, path_str_rep),
        }
    }

    pub fn to_wsfile(&self) -> NmideError<WSFile> {
        WSFile::new(&PathBuf::from(self.path.clone()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Folder {
    pub name: String,
    pub path: String,
    pub content: Vec<FolderOrFile>,
}

impl Folder {
    pub fn empty() -> Folder {
        Folder {
            name: String::new(),
            path: String::new(),
            content: Vec::new(),
        }
    }

    /// Creates a Folder instance
    ///
    /// Will not go any level deeper, ie. content will always be empty
    pub fn new(path: &Path) -> NmideError<Folder> {
        if !path.is_dir() {
            return NmideError {
                val: Folder::empty(),
                rep: Some(NmideReport {
                    msg: format!("The given path: `{path:?}` is not a directory"),
                    lvl: ErrorLevel::High,
                    tag: Vec::new(),
                    stack: Vec::new(),
                    origin: "Folder::new".to_string(),
                }),
            };
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

        let mut err = NmideError {
            val: Folder {
                name,
                path: path_str.unwrap_or_default(),
                content: Vec::new(),
            },
            rep: nmrep!(name_rep, extension_rep, path_str_rep),
        };

        err
    }
    /// Creates a WSFolder, with path 1
    pub fn to_wsfolder(self) -> NmideError<WSFolder> {
        WSFolder::new(&Path::new(&self.path), 0).or_else(|mut w| {
            let r = self
                .content
                .into_iter()
                .map(|f| -> Either<NmideError<WSFile>, NmideError<WSFolder>> {
                    match f {
                        FolderOrFile::File(f) => Either::Left(f.to_wsfile()),
                        FolderOrFile::Folder(f) => Either::Right(f.to_wsfolder()),
                    }
                })
                .map(|e| e.transpose())
                .fold(
                    NmideError {
                        val: Vec::new(),
                        rep: None,
                    },
                    |mut err, e| {
                        err.val.push(e.val);

                        if let Some(r) = e.rep {
                            err = err.push_nmide(r);
                        }

                        err
                    },
                );

            w.push_content(r.val);

            NmideError { val: w, rep: r.rep }
        })
    }
}
