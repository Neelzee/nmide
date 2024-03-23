use crate::{errors::NmideError, types, utils::funcs::os_to_str, workspace::ws::ws_file::WSFile};
use either::Either;
use eyre::{eyre, Context, OptionExt, Result};
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
