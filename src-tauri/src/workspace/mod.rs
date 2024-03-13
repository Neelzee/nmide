pub mod ws_file;

use crate::workspace::ws_file::WSFile;
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub struct Workspace<'a> {
    root: String,
    files: HashMap<&'a Path, WSFile>,
}
