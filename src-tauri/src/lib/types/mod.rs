pub mod modules;

use crate::{
    lib::{
        either::Either,
        errors::{ErrorLevel, NmideError, NmideReport},
        types::modules::{File, Folder, FolderOrFile},
        workspace::{ws_file::WSFile, ws_folder::WSFolder},
    },
    nmrep,
};
use std::ffi::OsString;
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
            name: OsString::new(),
            extension: OsString::new(),
            path: OsString::new(),
            symbol: String::new(),
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
        let name = path.file_name().unwrap_or_default().to_os_string();
        let extension = path.extension().unwrap_or_default().to_os_string();

        NmideError {
            val: File {
                name,
                extension,
                path: path.as_os_str().to_os_string(),
                symbol: String::new(),
            },
            rep: None,
        }
    }

    pub fn to_wsfile(&self) -> NmideError<WSFile> {
        WSFile::new(&PathBuf::from(self.path.clone()))
    }

    fn display(&self, indent: usize) -> String {
        format!("{}{:?}", " ".repeat(indent), self.name)
    }

    pub fn pretty_display(&self) -> String {
        self.display(2)
    }
}

impl Folder {
    pub fn pretty_display(&self) -> String {
        let indent: usize = 2;
        let mut str = String::new();
        for f in &self.content {
            match f {
                FolderOrFile::File(f) => str.push_str(&f.display(indent)),
                FolderOrFile::Folder(f) => str.push_str(&f.display(indent)),
            }
        }

        str
    }

    fn display(&self, indent: usize) -> String {
        let content = self
            .content
            .clone()
            .into_iter()
            .map(|e| match e {
                FolderOrFile::File(f) => f.display(indent),
                FolderOrFile::Folder(f) => f.display(indent),
            })
            .fold(String::new(), |s, f| s + &format!("\n{f}"));
        format!("{}{:?}\n{}", " ".repeat(indent), self.name, content)
    }

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
            name: OsString::new(),
            path: OsString::new(),
            content: Vec::new(),
            symbol: String::new(),
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

        let name = path.file_name().unwrap_or_default().to_os_string();

        NmideError {
            val: Folder {
                name,
                path: path.as_os_str().to_os_string(),
                content: Vec::new(),
                symbol: String::new(),
            },
            rep: None,
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
