use crate::{
    errors::NmideError,
    osops::{get_fof, get_paths},
    types,
    utils::funcs::os_to_str,
    workspace::ws_file::WSFile,
};
use either::Either;
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
    content: Vec<Either<WSFile, WSFolder>>,
}

impl WSFolder {
    pub fn new(path: &Path, level: usize) -> Result<Self> {
        Ok(Self {
            path: path.to_owned(),
            name: os_to_str(
                path.file_name()
                    .ok_or_eyre(format!("No filename from path: `{path:?}`"))?,
            )?,
            content: get_fof(path, level)?
                .into_iter()
                .map(|fof| -> Result<Either<WSFile, WSFolder>> {
                    match fof {
                        Either::Left(file) => {
                            let path = Path::new(&file.path);
                            Ok(Either::Left(WSFile::new(
                                &path.to_owned(),
                                Box::new(
                                    std::fs::File::open(path)
                                        .wrap_err(format!("Failed opening file: `{path:?}`"))?,
                                ),
                            )?))
                        }
                        Either::Right(folder) => {
                            let path = Path::new(&folder.path);
                            Ok(Either::Right(
                                WSFolder::new(path, level - 1)
                                    .wrap_err(format!("Failed opening file: `{path:?}`"))?,
                            ))
                        }
                    }
                })
                .fold(Vec::new(), |mut acc, fof| match fof {
                    Ok(f) => {
                        acc.push(f);
                        acc
                    }
                    Err(err) => {
                        debug!("Got error: `{err:?}`");
                        acc
                    }
                }),
        })
    }
}
