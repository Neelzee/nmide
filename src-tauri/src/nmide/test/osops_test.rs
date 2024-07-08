use crate::nmide::{
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

    let (dirs, _) = get_paths(path).unwrap_with_err();
    let count = dirs.len();
    let expected_count = FOLDER.len() - 1;

    println!("Expected: {expected_count}");
    println!("Count: {count}");

    assert_eq!(
        expected_count, count,
        "Expected: {expected_count}, got {count}, from: `{dirs:?}`"
    );
}

#[test]
fn test_get_folder_or_file_no_root() {
    let path = Path::new(TEST_PATH);

    let (dirs, _) = get_folder_or_file(path).unwrap_with_err();

    let paths = dirs
        .into_iter()
        .map(|f| match f {
            crate::nmide::either::Either::Left(f) => f.path,
            crate::nmide::either::Either::Right(f) => f.path,
        })
        .collect::<Vec<_>>();

    assert!(!paths.contains(&(path.as_os_str().to_owned())));
}

#[test]
fn test_get_folder_or_file_count() {
    let path = Path::new(TEST_PATH);

    let (dirs, _) = get_folder_or_file(path).unwrap_with_err();

    println!("{dirs:?}");
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
    let expected_count = FOLDER.len() - 1;

    assert_eq!(
        expected_count, count,
        "Expected: {expected_count}, got {count}, from: `{r:?}`"
    );
}
