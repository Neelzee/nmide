use super::{FOLDER, TEST_PATH};
use crate::lib::types::modules::FolderOrFile;
use crate::lib::utils::funcs::pretty_display;
use crate::lib::workspace::Workspace;
use std::path::Path;

#[test]
fn test_wsfile_creation() {
    let path = Path::new(TEST_PATH);

    let (ws, _) = Workspace::init(path).unwrap_with_err();

    println!(
        "{}",
        pretty_display(
            &vec![FolderOrFile::Folder(ws.to_folder().unwrap_with_err().0)],
            5
        )
    );
    let paths = ws
        .files
        .clone()
        .into_iter()
        .map(|f| -> String {
            match f {
                crate::lib::either::Either::Left(f) => f
                    .path
                    .clone()
                    .into_string()
                    .unwrap_or(format!("{:?}", f.path)),
                crate::lib::either::Either::Right(f) => f
                    .path
                    .clone()
                    .into_string()
                    .unwrap_or(format!("{:?}", f.path)),
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{paths}");

    assert_eq!(
        ws.len(),
        FOLDER.len(),
        "Failed, ws: \n{:?}\n, \nFOLDER\n{:?}",
        paths,
        FOLDER
    );
    assert_eq!(ws.to_folder().unwrap_with_err().0.len(), FOLDER.len(),);
}
