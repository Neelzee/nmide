use crate::{
    lib::{
        either::Either,
        errors::NmideError,
        osops::get_folder_or_file,
        types::modules::{self, FolderOrFile},
        workspace::ws_file::WSFile,
    },
    nmrep,
};
use std::ffi::OsString;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct WSFolder {
    path: OsString,
    name: OsString,
    content: Vec<Either<WSFolder, WSFile>>,
}

impl WSFolder {
    pub fn new(path: &Path, level: usize) -> NmideError<Self> {
        let name = path.file_name().unwrap_or_default().to_os_string();
        let (raw_content, raw_content_rep) = get_folder_or_file(path, level).unwrap_with_err();

        let (content, content_rep) = raw_content
            .into_iter()
            .map(|f| match f {
                Either::Left(f) => Either::Left(f.to_wsfolder()),
                Either::Right(f) => Either::Right(f.to_wsfile()),
            })
            .fold(
                NmideError {
                    val: Vec::new(),
                    rep: None,
                },
                |mut err, either| {
                    let e = either.transpose();
                    err.val.push(e.val);

                    if let Some(rep) = e.rep {
                        err = err.push_nmide(rep);
                    }

                    err
                },
            )
            .unwrap_with_err();

        let ws = WSFolder {
            path: path.as_os_str().to_os_string(),
            name,
            content,
        };

        NmideError {
            val: ws,
            rep: nmrep!(raw_content_rep, content_rep),
        }
    }

    pub fn get_content(&self) -> NmideError<Vec<FolderOrFile>> {
        self.content
            .iter()
            .map(|v| match v {
                Either::Right(f) => f
                    .to_file()
                    .map(|val| NmideError::new(FolderOrFile::File(val))),
                Either::Left(f) => f
                    .to_folder()
                    .map(|val| NmideError::new(FolderOrFile::Folder(val))),
            })
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

    pub fn to_folder(&self) -> NmideError<modules::Folder> {
        self.get_content().map(|val| {
            NmideError::new(modules::Folder {
                name: OsString::from(self.name.clone()),
                path: self.path.as_os_str().to_os_string(),
                content: val,
            })
        })
    }

    pub fn push_content(&mut self, mut content: Vec<Either<WSFolder, WSFile>>) {
        self.content.append(&mut content);
    }
}
