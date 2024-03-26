use eyre::{Context, OptionExt, Result};
use std::fs::File;
use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::{
    either::Either,
    errors::{ErrorLevel, NmideError, NmideReport},
    nmrep,
    types::{self, FolderOrFile},
    utils::funcs::os_to_str,
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

/// Returns a list of all the paths in the given directory
///
/// Recursively checks for until level = 0
pub fn get_paths(path: &Path, level: usize) -> NmideError<Vec<PathBuf>> {
    visit_dirs_recursive(path, level).vmap(|v| {
        v.into_iter()
            .map(|e| match e {
                Either::Left(f) => PathBuf::from(f.path),
                Either::Right(f) => PathBuf::from(f.path),
            })
            .collect()
    })
}

pub fn get_folder_or_file(
    path: &Path,
    level: usize,
) -> NmideError<Vec<Either<types::File, types::Folder>>> {
    visit_dirs_recursive(path, level)
}

fn visit_dirs_recursive(
    dir: &Path,
    depth: usize,
) -> NmideError<Vec<Either<types::File, types::Folder>>> {
    let mut paths = Vec::new();

    if depth == 0 {
        return NmideError {
            val: Some(paths),
            rep: None,
        };
    }

    let mut err = NmideError {
        val: None,
        rep: None,
    };

    if dir.is_dir() {
        let name = os_to_str(dir.file_name().unwrap_or_default());

        let path_str = NmideError {
            val: dir.to_str().and_then(|p| Some(p.to_string())),
            rep: Some(NmideReport {
                msg: format!("Failed converting Path to String: `{dir:?}`"),
                lvl: ErrorLevel::Low,
                tag: Vec::new(),
                stack: Vec::new(),
                origin: "visit_dirs_recursive".to_string(),
            }),
        };

        let content = visit_dirs_recursive(dir, depth - 1).and_then(|mut e| {
            Some(
                e.into_iter()
                    .map(std::convert::Into::into)
                    .collect::<Vec<FolderOrFile>>(),
            )
        });

        let (name, name_rep) = name.unwrap_with_err();
        let (path_str, path_str_rep) = path_str.unwrap_with_err();
        let (content, content_rep) = content.unwrap_with_err();

        err.rep = nmrep!(name_rep, path_str_rep, content_rep);

        paths.push(Either::Right(types::Folder {
            name: name.unwrap_or_default(),
            path: path_str.unwrap_or_default(),
            content: content.unwrap_or_default(),
        }));
    } else {
        let (name, name_rep) = os_to_str(dir.file_name().unwrap_or_default()).unwrap_with_err();

        let (path_str, path_str_rep) = NmideError {
            val: dir.to_str().and_then(|p| Some(p.to_string())),
            rep: Some(NmideReport {
                msg: format!("Failed converting Path to String: `{dir:?}`"),
                lvl: ErrorLevel::Low,
                tag: Vec::new(),
                stack: Vec::new(),
                origin: "_visit_dirs_recursive".to_string(),
            }),
        }
        .unwrap_with_err();

        let (extension, extension_rep) =
            os_to_str(dir.extension().unwrap_or_default()).unwrap_with_err();

        err.rep = nmrep!(name_rep, path_str_rep, extension_rep);

        paths.push(Either::Left(types::File {
            name: name.unwrap_or_default(),
            path: path_str.unwrap_or_default(),
            extension: extension.unwrap_or_default(),
            content: None::<String>,
        }));
    }

    err.val = Some(paths);

    err
}
