use once_cell::unsync::Lazy;

use crate::{
    osops::{get_folder_or_file, get_paths},
    types::modules::{File, Folder, FolderOrFile},
};

use std::path::{Path, PathBuf};

mod osops_test;
mod type_test;
mod utils_test;
mod workspace_test;

const TEST_PATH: &str = "../../../.test/";

const FOLDER: Lazy<FolderOrFile> = Lazy::new(|| {
    FolderOrFile::Folder(Folder {
        name: ".test".to_string(),
        path: format!("TODO"),
        content: vec![
            FolderOrFile::Folder(Folder {
                name: "packages".to_string(),
                path: format!("{TEST_PATH}/"),
                content: vec![
                    FolderOrFile::File(File {
                        name: "bar.c".to_string(),
                        extension: "c".to_string(),
                        path: format!("{TEST_PATH}/packges/"),
                    }),
                    FolderOrFile::File(File {
                        name: "foo.mg".to_string(),
                        extension: "mg".to_string(),
                        path: format!("{TEST_PATH}/packges/"),
                    }),
                    FolderOrFile::File(File {
                        name: "foobar.rs".to_string(),
                        extension: "rs".to_string(),
                        path: format!("{TEST_PATH}/packges/"),
                    }),
                    FolderOrFile::File(File {
                        name: "main".to_string(),
                        extension: "".to_string(),
                        path: format!("{TEST_PATH}/packges/"),
                    }),
                    FolderOrFile::File(File {
                        name: "main.c".to_string(),
                        extension: "c".to_string(),
                        path: format!("{TEST_PATH}/packges/"),
                    }),
                ],
            }),
            FolderOrFile::File(File {
                name: "1.mg".to_string(),
                extension: "mg".to_string(),
                path: format!("{TEST_PATH}/"),
            }),
            FolderOrFile::File(File {
                name: "nmide.mg".to_string(),
                extension: "mg".to_string(),
                path: format!("{TEST_PATH}/"),
            }),
        ],
    })
});
