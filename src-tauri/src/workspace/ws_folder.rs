use crate::{
    either::Either,
    errors::NmideError,
    nmrep,
    osops::{get_folder_or_file, get_paths},
    types::{
        self,
        modules::{self, FolderOrFile},
    },
    utils::funcs::os_to_str,
    workspace::ws_file::WSFile,
};
use eyre::{eyre, Context, OptionExt, Result};
use log::debug;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct WSFolder {
    path: PathBuf,
    name: String,
    content: Vec<Either<WSFolder, WSFile>>,
}

impl WSFolder {
    pub fn new(path: &Path, level: usize) -> NmideError<Self> {
        let (name, name_rep) = os_to_str(path.file_name().unwrap_or_default()).unwrap_with_err();
        let (mut raw_content, raw_content_rep) = get_folder_or_file(path, level).unwrap_with_err();

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
            path: path.to_owned(),
            name,
            content,
        };

        NmideError {
            val: ws,
            rep: nmrep!(name_rep, raw_content_rep, content_rep),
        }
    }

    pub fn get_content(&self) -> NmideError<Vec<FolderOrFile>> {
        (&self.content)
            .into_iter()
            .map(|v| match v {
                Either::Right(f) => f.to_file().map(|e| NmideError {
                    val: FolderOrFile::File(e.val),
                    rep: e.rep,
                }),
                Either::Left(f) => f.to_folder().map(|e| NmideError {
                    val: FolderOrFile::Folder(e.val),
                    rep: e.rep,
                }),
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
        self.get_content().map(|err| NmideError {
            val: modules::Folder {
                name: self.name.clone(),
                path: self.path.to_str().unwrap_or_default().to_string(),
                content: err.val,
            },
            rep: err.rep,
        })
    }

    pub fn push_content(&mut self, mut content: Vec<Either<WSFolder, WSFile>>) {
        self.content.append(&mut content);
    }
}
