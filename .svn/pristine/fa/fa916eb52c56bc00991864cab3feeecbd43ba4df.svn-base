use crate::{
    either::Either,
    errors::{ErrorLevel, NmideError, NmideReport},
    types::modules::{File, Folder, FolderOrFile},
};
use std::{ffi::OsStr, path::PathBuf};

pub fn os_to_str(s: &OsStr) -> NmideError<String> {
    NmideError {
        val: s
            .to_str().map(|s| s.to_string())
            .unwrap_or(format!("{s:?}")),
        rep: Some(NmideReport {
            msg: format!("Failed converting String: `{s:?}`"),
            lvl: ErrorLevel::Low,
            tag: Vec::new(),
            stack: Vec::new(),
            origin: "os_to_str".to_string(),
        }),
    }
}

pub fn pretty_display(files: &Vec<FolderOrFile>, lvl: usize) -> String {
    let mut s = String::new();

    for f in files {
        match f {
            FolderOrFile::File(f) => s += &format!("{}{}\n", " ".repeat(lvl), f.name),
            FolderOrFile::Folder(f) => {
                s += &format!("{}{}\n", " ".repeat(lvl), f.name);
                s += &pretty_display(&f.content, lvl + 2);
            }
        }
    }

    s
}

pub fn to_paths(vec: Vec<Either<Folder, File>>) -> Vec<PathBuf> {
    vec.into_iter().fold(Vec::new(), |mut acc, e| {
        match e {
            Either::Right(f) => acc.push(PathBuf::from(f.path)),
            Either::Left(f) => {
                acc.push(PathBuf::from(f.path));
                acc.append(&mut to_paths(
                    f.content
                        .into_iter()
                        .map(|e| e.into())
                        .collect::<Vec<Either<_, _>>>(),
                ));
            }
        }
        acc
    })
}
