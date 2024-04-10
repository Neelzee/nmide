use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum FolderOrFile {
    File(File),
    Folder(Folder),
}

impl Serialize for FolderOrFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FolderOrFile::File(f) => {
                let mut state = serializer.serialize_struct("File", 3)?;
                state.serialize_field("name", &f.name)?;
                state.serialize_field("extension", &f.extension)?;
                state.serialize_field("path", &f.path)?;
                state.end()
            }
            FolderOrFile::Folder(f) => {
                let mut state = serializer.serialize_struct("Folder", 4)?;
                state.serialize_field("name", &f.name)?;
                state.serialize_field("path", &f.path)?;
                state.serialize_field("content", &f.content)?;
                state.end()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Folder {
    pub name: String,
    pub path: String,
    pub content: Vec<FolderOrFile>,
}

#[derive(Debug, Clone, Serialize)]
pub struct File {
    pub name: String,
    pub extension: String,
    pub path: String,
}
