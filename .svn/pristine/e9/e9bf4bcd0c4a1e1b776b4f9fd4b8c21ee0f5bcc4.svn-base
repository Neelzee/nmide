use anyhow::{anyhow, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub struct FOptions {
    depth: usize,
    ignore_hidden: bool,
    ignore: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum Fo {
    File(String),
    Folder(String, Vec<Fo>),
}

impl FOptions {
    pub fn new(depth: usize, ignore: Vec<PathBuf>, ignore_hidden: bool) -> Self {
        Self {
            depth,
            ignore_hidden,
            ignore,
        }
    }
}

impl Default for FOptions {
    fn default() -> Self {
        Self {
            depth: 3,
            ignore_hidden: true,
            ignore: Vec::new(),
        }
    }
}

pub(crate) fn walk_dir(path: PathBuf, options: FOptions) -> Result<Option<Fo>> {
    Ok(walk(
        path,
        options.ignore_hidden,
        options.depth,
        &options.ignore,
    )?)
}

fn walk(pth: PathBuf, ignore_hidden: bool, depth: usize, ignore: &[PathBuf]) -> Result<Option<Fo>> {
    if ignore_hidden && pth.starts_with(".") {
        return Ok(None);
    }
    if ignore.contains(&pth) {
        return Ok(None);
    }
    if depth == 0 {
        return Ok(None);
    }
    let name = pth
        .to_str()
        .ok_or(anyhow!("Couldnt stringify path: {:?}", pth))?
        .to_string();
    if pth.is_dir() {
        Ok(Some(Fo::Folder(
            name,
            pth.read_dir()?
                .flat_map(|f| {
                    f.map(|d| walk(d.path(), ignore_hidden, depth - 1, ignore).unwrap_or_default())
                        .unwrap_or_default()
                })
                .collect(),
        )))
    } else {
        Ok(Some(Fo::File(name)))
    }
}
