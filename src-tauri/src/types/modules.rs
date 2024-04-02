use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FolderOrFile {
    File(File),
    Folder(Folder),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Folder {
    pub name: String,
    pub path: String,
    pub content: Vec<FolderOrFile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub extension: String,
    pub path: String,
}
