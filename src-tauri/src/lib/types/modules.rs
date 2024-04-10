use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::ffi::OsString;

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
                state.serialize_field(
                    "name",
                    &(f.name.clone().into_string().ok().unwrap_or_default()),
                )?;
                state.serialize_field(
                    "extension",
                    &(f.extension.clone().into_string().ok().unwrap_or_default()),
                )?;
                state.serialize_field(
                    "path",
                    &(f.path.clone().into_string().ok().unwrap_or_default()),
                )?;
                state.end()
            }
            FolderOrFile::Folder(f) => {
                let mut state = serializer.serialize_struct("Folder", 4)?;
                state.serialize_field(
                    "name",
                    &(f.name.clone().into_string().ok().unwrap_or_default()),
                )?;
                state.serialize_field(
                    "path",
                    &(f.path.clone().into_string().ok().unwrap_or_default()),
                )?;
                state.serialize_field("content", &f.content)?;
                state.end()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Folder {
    pub name: OsString,
    pub path: OsString,
    pub content: Vec<FolderOrFile>,
}

#[derive(Debug, Clone, Serialize)]
pub struct File {
    pub name: OsString,
    pub extension: OsString,
    pub path: OsString,
}
