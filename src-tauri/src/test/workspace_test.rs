use super::{FOLDER, TEST_PATH};
use crate::types::modules::FolderOrFile;
use crate::utils::funcs::pretty_display;
use crate::workspace::Workspace;
use std::path::Path;

#[test]
fn test_wsfile_creation() {
    let path = Path::new(TEST_PATH);

    let (ws, _) = Workspace::init(path).unwrap_with_err();

    assert_eq!(ws.len(), FOLDER.len());
    println!(
        "{}",
        pretty_display(
            &vec![FolderOrFile::Folder(ws.to_folder().unwrap_with_err().0)],
            5
        )
    );
    let f: FolderOrFile = FOLDER.clone();
    println!("{}", pretty_display(&vec![f], 5));
    assert_eq!(ws.to_folder().unwrap_with_err().0.len(), FOLDER.len(),);
}
