use crate::nmide::types::modules::{File, Folder, FolderOrFile};
use once_cell::unsync::Lazy;
use std::ffi::OsString;

mod osops_test;
mod type_test;
mod utils_test;
mod workspace_test;

const TEST_PATH: &str = "/home/nmf/Documents/nmide/.test/";

const FOLDER: Lazy<FolderOrFile> = Lazy::new(|| {
    FolderOrFile::Folder(Folder {
        name: OsString::from(".test"),
        path: OsString::from("TODO"),
        symbol: String::new(),
        content: vec![
            FolderOrFile::Folder(Folder {
                name: OsString::from("packages"),
                path: OsString::from("{TEST_PATH}/"),
                symbol: String::new(),
                content: vec![
                    FolderOrFile::File(File {
                        name: OsString::from("bar.c"),
                        extension: OsString::from("c"),
                        path: OsString::from("{TEST_PATH}/packages/"),
                        symbol: String::new(),
                    }),
                    FolderOrFile::File(File {
                        name: OsString::from("foo.mg"),
                        extension: OsString::from("mg"),
                        path: OsString::from("{TEST_PATH}/packages/"),
                        symbol: String::new(),
                    }),
                    FolderOrFile::File(File {
                        name: OsString::from("foobar.rs"),
                        extension: OsString::from("rs"),
                        path: OsString::from("{TEST_PATH}/packages/"),
                        symbol: String::new(),
                    }),
                    FolderOrFile::File(File {
                        name: OsString::from("main"),
                        extension: OsString::from(""),
                        path: OsString::from("{TEST_PATH}/packages/"),
                        symbol: String::new(),
                    }),
                    FolderOrFile::File(File {
                        name: OsString::from("main.c"),
                        extension: OsString::from("c"),
                        path: OsString::from("{TEST_PATH}/packages/"),
                        symbol: String::new(),
                    }),
                ],
            }),
            FolderOrFile::File(File {
                name: OsString::from("1.mg"),
                extension: OsString::from("mg"),
                path: OsString::from("{TEST_PATH}/"),
                symbol: String::new(),
            }),
            FolderOrFile::File(File {
                name: OsString::from("nmide.mg"),
                extension: OsString::from("mg"),
                path: OsString::from("{TEST_PATH}/"),
                symbol: String::new(),
            }),
        ],
    })
});
