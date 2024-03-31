use std::{fs::File, path::Path};

use eyre::{Context, Result};

use super::{FOLDER, TEST_PATH};
use crate::workspace::Workspace;

#[test]
fn test_wsfile_creation() {
    let path = Path::new(TEST_PATH);

    let (ws, _) = Workspace::init(path).unwrap_with_err();

    println!("{ws:?}");
    println!("{:?}", ws.get_files());

    assert!(ws.get_files().len() > 1);
}
