use either::Either;
use eyre::{Context, OptionExt, Result};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::types::{self, FolderOrFile};
use crate::utils::funcs::os_to_str;

/// Reads from the file into a buffer
pub fn read_file(file: &File) -> Result<String> {
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .wrap_err("Failed reading from file into buffer")?;
    Ok(buffer)
}

// Writes content onto file
pub fn write_to_file(file: &File, content: String) -> Result<()> {
    let mut writer = BufWriter::new(file);
    writer
        .write_all(content.as_bytes())
        .wrap_err("Failed writing content into file")?;
    Ok(())
}

fn visit_dirs_recursive(dir: &Path, depth: usize, paths: &mut Vec<PathBuf>) -> Result<()> {
    paths.push(dir.to_owned());
    if depth == 0 {
        return Ok(());
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            paths.push(path.clone());

            if path.is_dir() {
                visit_dirs_recursive(&path, depth - 1, paths)?;
            }
        }
    }

    Ok(())
}

/// Returns a list of all the paths in the given directory
///
/// Recursively checks for until level = 0
pub fn get_paths(path: &Path, level: usize) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    visit_dirs_recursive(path, level, &mut paths)?;
    Ok(paths)
}

pub fn get_fof(path: &Path, level: usize) -> Result<Vec<Either<types::File, types::Folder>>> {
    _visit_dirs_recursive(path, level)
}

fn _visit_dirs_recursive(
    dir: &Path,
    depth: usize,
) -> Result<Vec<Either<types::File, types::Folder>>> {
    let mut paths = Vec::new();

    if depth == 0 {
        return Ok(paths);
    }

    if dir.is_dir() {
        paths.push(Either::Right(types::Folder {
            name: os_to_str(
                dir.file_name()
                    .ok_or_eyre(format!("Failed getting filename from: `{dir:?}`"))?,
            )?,
            path: dir
                .to_str()
                .ok_or_eyre(format!("Failed reading path as UTF-8 String: `{dir:?}`"))?
                .to_string(),
            content: _visit_dirs_recursive(dir, depth - 1)?
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<FolderOrFile>>(),
        }));
    } else {
        paths.push(Either::Left(types::File {
            name: os_to_str(
                dir.file_name()
                    .ok_or_eyre(format!("Failed getting filename from: `{dir:?}`"))?,
            )?,
            extension: dir
                .extension()
                .and_then(|os| os_to_str(os).ok())
                .or_else(|| Some("".to_string()))
                .unwrap(),
            path: dir
                .to_str()
                .ok_or_eyre(format!("Failed reading path as UTF-8 String: `{dir:?}`"))?
                .to_string(),
            content: None::<String>,
        }))
    }

    Ok(paths)
}
