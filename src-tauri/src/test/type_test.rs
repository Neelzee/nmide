use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{errors, types::modules};

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

    let str_nmide_err = r#"export type NmideError<T> = {
  val: T,
  rep: NmideReport | null
};"#
    .to_string();

    let str_nmide_rep = format!(
        "export type NmideReport = {};",
        serde_json::to_string_pretty(&errors::NmideReport {
            msg: "string".to_string(),
            lvl: errors::ErrorLevel::Low,
            tag: Vec::new(),
            stack: Vec::new(),
            origin: "string".to_string(),
        })?
    )
    .replace("Low", "ErrorLevel")
    .replacen("tag: []", "tag: string[]", 1)
    .replacen("stack: []", "stack: NmideReport[]", 1)
    .replace("\"", "");

    let str_error_lvl = r#"export enum ErrorLevel {
  Low,
  Medium,
  High,
  Unknown
};"#
    .to_string();

    writer.write_all(
        format!("{str_folder_or_file}\n\n{str_file}\n\n{str_folder}\n\n{str_nmide_err}\n\n{str_nmide_rep}\n\n{str_error_lvl}").as_bytes(),
    )?;

    Ok(())
}
