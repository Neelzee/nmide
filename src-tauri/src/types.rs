use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Fof {
    File(File),
    Folder(Folder),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "file")]
pub struct File {
    name: String,
    extension: String,
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "folder")]
pub struct Folder {
    name: String,
    extension: String,
    path: String,
    content: Vec<Fof>,
}
