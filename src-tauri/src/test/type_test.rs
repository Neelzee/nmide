use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::types::modules;

use eyre::Result;

#[test]
fn type_test() -> Result<()> {
    let file: File;

    if cfg!(windows) {
        file = File::create("..\\src\\types.ts")?;
    } else {
        file = File::create("../src/types.ts")?;
    }

    let mut writer = BufWriter::new(file);

    let str_file = format!(
        "export type File = {};",
        serde_json::to_string_pretty(&modules::File {
            name: "string".to_string(),
            extension: "string".to_string(),
            path: "string".to_string(),
        })?
    )
    .replace("\"", "");

    let str_folder = format!(
        "export type Folder = {};",
        serde_json::to_string_pretty(&modules::Folder {
            name: "string".to_string(),
            path: "string".to_string(),
            content: Vec::new(),
        })?
    )
    .replace("\"", "")
    .replace("[]", "FolderOrFile[]");

    let str_folder_or_file = format!("export type FolderOrFile = Folder | File;");

    writer.write_all(format!("{str_folder_or_file}\n\n{str_file}\n\n{str_folder}").as_bytes())?;

    Ok(())
}
