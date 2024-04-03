use std::path::Path;

use super::{FOLDER, TEST_PATH};
use crate::workspace::Workspace;

#[test]
fn test_wsfile_creation() {
    let path = Path::new(TEST_PATH);

    let (ws, _) = Workspace::init(path).unwrap_with_err();

    assert_eq!(ws.len(), FOLDER.len())
}
