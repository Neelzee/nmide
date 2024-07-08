use super::FOLDER;
use crate::nmide::{types::modules::FolderOrFile, utils::funcs::pretty_display};

#[test]
fn test_pretty_display() {
    let vec: Vec<FolderOrFile>;
    let f: &FolderOrFile = &FOLDER;
    match f {
        FolderOrFile::File(f) => {
            vec = vec![FolderOrFile::File(f.clone())];
            unreachable!("FOLDER should always be a Folder");
        }
        FolderOrFile::Folder(f) => {
            vec = vec![FolderOrFile::Folder(f.clone())];
        }
    }

    println!("{}", pretty_display(&vec, 0));

    assert!(true);
}
