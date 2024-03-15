pub mod ws_file;

use crate::workspace::ws_file::WSFile;
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub struct Workspace<'a> {
    root: &'a Path,
    files: HashMap<&'a Path, WSFile<'a>>,
}

impl Workspace<'static> {
    pub fn new<'a>(root: &'a Path, files: HashMap<&'a Path, WSFile<'a>>) -> Workspace<'a> {
        Workspace { root, files }
    }
}
