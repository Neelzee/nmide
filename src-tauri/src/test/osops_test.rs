use crate::osops::{get_folder_or_file, get_paths};
use crate::test::{FOLDER, TEST_PATH};

use std::path::{Path, PathBuf};

#[test]
fn test_path_getter_no_root() {
    let path = Path::new(TEST_PATH);

    let p = PathBuf::from(TEST_PATH);

    let (dirs, dirs_rep) = get_paths(path, 2).unwrap_with_err();

    println!("{dirs:?}");
    println!("\n{dirs_rep:?}");

    assert!(!dirs.contains(&p));
}

/// Test that the path count is correct
#[test]
fn test_get_paths_count() {
    let path = Path::new(TEST_PATH);

    let (dirs, _) = get_paths(path, 2).unwrap_with_err();

    let count = dirs.len();
    let expected_count = FOLDER.len();
    println!("Expected: {expected_count}, got {count}, from: `{dirs:?}`");
    assert_eq!(
        expected_count, count,
        "Expected: {expected_count}, got {count}, from: `{dirs:?}`"
    );
}

#[test]
fn test_path_getter() {
    let path = Path::new(TEST_PATH);

    let (dirs, dirs_rep) = get_paths(path, 2).unwrap_with_err();

    println!("{dirs:?}");
    println!("\n{dirs_rep:?}");

    assert!(dirs.len() > 1);
}

#[test]
fn test_get_folder_or_file() {
    let path = Path::new(TEST_PATH);

    let (fofs, r) = get_folder_or_file(path, 2).unwrap_with_err();

    println!("{r:?}");
    println!("{fofs:?}");

    assert!(fofs.len() > 1);
}
