use crate::lib::{
    osops::get_paths,
    test::{FOLDER, TEST_PATH},
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
