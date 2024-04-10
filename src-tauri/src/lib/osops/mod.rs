use eyre::{Context, Result};
use log::warn;
use std::collections::HashSet;
use std::fs::{read_dir, File};
use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::{
    lib::{
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
///
/// Recursively checks for until level = 0
pub fn get_paths(path: &Path, level: usize) -> NmideError<Vec<PathBuf>> {
    visit_dirs_recursive(path, level)
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

pub fn get_folder_or_file(
    path: &Path,
    level: usize,
) -> NmideError<Vec<Either<modules::Folder, modules::File>>> {
    visit_dirs_recursive(path, level)
}

fn visit_dirs_recursive(
    dir: &Path,
    depth: usize,
) -> NmideError<Vec<Either<modules::Folder, modules::File>>> {
    let mut res = NmideError {
        val: Vec::new(),
        rep: None,
    };

    if depth == 0 {
        return res;
    }

    if dir.is_dir() {
        let name = dir.file_name().unwrap_or_default().to_os_string();
        let path_str = dir.as_os_str().to_os_string();

        // TODO: This does not work
        let (content, content_rep) = NmideError::from_err(read_dir(dir))
            .map(|sub_dir| -> NmideError<Vec<FolderOrFile>> {
                match sub_dir {
                    Some(sd) => fold_nmide(
                        sd.map(|e| match e {
                            Ok(p) => NmideError {
                                val: p.path(),
                                rep: None,
                            },
                            Err(err) => NmideError {
                                val: PathBuf::new(),
                                rep: Some(NmideReport::from_err(err)),
                            },
                        })
                        .map(|p| {
                            if p.val.is_dir() {
                                visit_dirs_recursive(p.val.as_path(), depth - 1)
                                    .vmap(|vc| {
                                        vc.into_iter()
                                            .map(|e| e.into())
                                            .collect::<Vec<FolderOrFile>>()
                                    })
                                    .map(|val| {
                                        modules::Folder::new(p.val.as_path()).vmap(|mut f| {
                                            f.content = val;
                                            FolderOrFile::Folder(f)
                                        })
                                    })
                            } else {
                                p.map(|val| {
                                    modules::File::new(val.as_path()).vmap(FolderOrFile::File)
                                })
                            }
                        })
                        .collect(),
                    ),
                    None => NmideError {
                        val: Vec::new(),
                        rep: None,
                    },
                }
            })
            .unwrap_with_err();

        res.val.push(Either::Left(modules::Folder {
            name,
            path: path_str,
            content,
        }));

        res.rep = nmrep!(res.rep, content_rep);
    } else {
        let name = dir.file_name().unwrap_or_default().to_os_string();
        let path_str = dir.as_os_str().to_os_string();

        let extension = dir.extension().unwrap_or_default().to_os_string();

        res.val.push(Either::Right(modules::File {
            name,
            path: path_str,
            extension,
        }));
    }

    res
}
