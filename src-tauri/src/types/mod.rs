pub mod modules;

use crate::{
    either::Either,
    errors::{ErrorLevel, NmideError, NmideReport},
    nmrep,
    types::modules::{File, Folder, FolderOrFile},
    utils::funcs::os_to_str,
    workspace::{ws_file::WSFile, ws_folder::WSFolder},
};
use std::path::{Path, PathBuf};

impl From<Either<Folder, File>> for FolderOrFile {
    fn from(value: Either<Folder, File>) -> Self {
        match value {
            Either::Right(f) => FolderOrFile::File(f),
            Either::Left(f) => FolderOrFile::Folder(f),
        }
    }
}

impl From<FolderOrFile> for Either<Folder, File> {
    fn from(value: FolderOrFile) -> Self {
        match value {
            FolderOrFile::File(f) => Either::Right(f),
            FolderOrFile::Folder(f) => Either::Left(f),
        }
    }
}

impl FolderOrFile {
    pub fn len(&self) -> usize {
        match self {
            FolderOrFile::File(_) => 1,
            FolderOrFile::Folder(f) => 1 + f.content.iter().fold(0, |c, f| c + f.len()),
        }
    }
}

impl File {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            extension: String::new(),
            path: String::new(),
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
            val: path.to_str().map(|s| s.to_string()),
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
            },
            rep: nmrep!(name_rep, extension_rep, path_str_rep),
        }
    }

    pub fn to_wsfile(&self) -> NmideError<WSFile> {
        WSFile::new(&PathBuf::from(self.path.clone()))
    }
}

impl Folder {
    pub fn len(&self) -> usize {
        1 + self.content.clone().into_iter().fold(0, |mut i, f| {
            match f {
                FolderOrFile::File(_) => i += 1,
                FolderOrFile::Folder(folder) => i += folder.len(),
            }
            i
        })
    }

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

        let (path_str, path_str_rep) = NmideError {
            val: path.to_str().map(|s| s.to_string()),
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
            val: Folder {
                name,
                path: path_str.unwrap_or_default(),
                content: Vec::new(),
            },
            rep: nmrep!(name_rep, path_str_rep),
        }
    }

    /// Creates a WSFolder, with path 1
    pub fn to_wsfolder(self) -> NmideError<WSFolder> {
        WSFolder::new(Path::new(&self.path), 0).or_else(|mut w| {
            let r = self
                .content
                .into_iter()
                .map(|f| -> Either<NmideError<WSFolder>, NmideError<WSFile>> {
                    match f {
                        FolderOrFile::File(f) => Either::Right(f.to_wsfile()),
                        FolderOrFile::Folder(f) => Either::Left(f.to_wsfolder()),
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
