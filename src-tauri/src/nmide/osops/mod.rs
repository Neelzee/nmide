use eyre::{Context, Result};
use log::warn;
use std::collections::HashSet;
use std::fs::{read_dir, File};
use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::nmfold;
use crate::{
    nmide::{
        either::Either,
        errors::{fold_nmide, ErrorLevel, NmideError, NmideReport},
        types::{modules, modules::FolderOrFile},
        utils::funcs::{os_to_str, to_paths},
    },
    nmrep,
};

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
pub fn write_to_file(file: &File, content: &str) -> Result<()> {
    let mut writer = BufWriter::new(file);
    writer
        .write_all(content.as_bytes())
        .wrap_err("Failed writing content into file")?;
    Ok(())
}

/// Returns a list of all the absolute paths in the given directory
pub fn get_paths(path: &Path) -> NmideError<Vec<PathBuf>> {
    visit_dirs_recursive(path)
        .vmap(|v| {
            v.into_iter().fold(Vec::new(), |mut acc, e| match e {
                Either::Right(f) => {
                    acc.push(PathBuf::from(f.path));
                    acc
                }
                Either::Left(f) => {
                    acc.push(PathBuf::from(f.path));
                    acc.append(&mut to_paths(
                        f.content.into_iter().map(|e| e.into()).collect(),
                    ));
                    acc
                }
            })
        }) // Ensures no duplicates
        .vmap(|e| e.into_iter().collect::<HashSet<_>>().into_iter().collect())
}

pub fn get_folder_or_file(path: &Path) -> NmideError<Vec<Either<modules::Folder, modules::File>>> {
    visit_dirs_recursive(path)
}

fn visit_dirs_recursive(dir: &Path) -> NmideError<Vec<Either<modules::Folder, modules::File>>> {
    if dir.is_dir() {
        NmideError::from_err(read_dir(dir).map(|e| {
            nmfold!(e
                .filter_map(|p| p.ok())
                .map(
                    |p| -> Either<NmideError<modules::Folder>, NmideError<modules::File>> {
                        let sub_path = p.path();

                        if sub_path.is_dir() {
                            Either::Left(visit_dirs_recursive(&sub_path).vmap(|content| {
                                modules::Folder {
                                    name: sub_path.file_name().unwrap_or_default().to_os_string(),
                                    path: sub_path.as_os_str().to_os_string(),
                                    content: content.into_iter().map(|e| e.into()).collect(),
                                    symbol: String::new(),
                                }
                            }))
                        } else {
                            Either::Right(modules::File::new(&sub_path))
                        }
                    },
                )
                .map(|e| e.transpose())
                .collect::<Vec<_>>())
        }))
        .option_combine()
        .map(|v| match v {
            Some(v) => NmideError::new(v),
            None => NmideError::new(Vec::new()).push_nmide(NmideReport::new(
                format!("Failed getting content from: `{dir:?}`"),
                format!("visit_dirs_recursive({dir:?})"),
            )),
        })
    } else {
        NmideError::new(vec![Either::Right(modules::File {
            name: dir.file_name().unwrap_or_default().to_os_string(),
            extension: dir.extension().unwrap_or_default().to_os_string(),
            path: dir.as_os_str().to_os_string(),
            symbol: String::new(),
        })])
    }
}
