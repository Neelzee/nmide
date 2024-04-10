use crate::lib::{
    osops::{get_folder_or_file, get_paths},
    test::{FOLDER, TEST_PATH},
    types::modules::{Folder, FolderOrFile},
    utils::funcs::pretty_display,
};

use std::path::Path;

/// Test that the path count is correct
#[test]
fn test_get_paths_count() {
    let path = Path::new(TEST_PATH);

    let (dirs, _) = get_paths(path, 2).unwrap_with_err();
    let count = dirs.len();
    let expected_count = FOLDER.len();

    println!("Expected: {expected_count}");
    println!("Count: {count}");

    assert_eq!(
        expected_count, count,
        "Expected: {expected_count}, got {count}, from: `{dirs:?}`"
    );
}

#[test]
fn test_get_folder_or_file_count() {
    let path = Path::new(TEST_PATH);

    let (dirs, _) = get_folder_or_file(path, 2).unwrap_with_err();

    let r = dirs
        .clone()
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<FolderOrFile>>();

    println!("{}", pretty_display(&r, 5));

    println!("=========================");
    let f: FolderOrFile = FOLDER.clone();
    println!("{}", pretty_display(&vec![f], 5));

    let count = r.clone().into_iter().fold(0, |i, f| i + f.len());
    let expected_count = FOLDER.len();

    assert_eq!(
        expected_count, count,
        "Expected: {expected_count}, got {count}, from: `{r:?}`"
    );
}
