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
                state.serialize_field("symbol", &f.symbol)?;
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
                state.serialize_field("symbol", &f.symbol)?;
                state.end()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub name: OsString,
    pub path: OsString,
    pub content: Vec<FolderOrFile>,
    pub symbol: String,
}

impl Serialize for Folder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Folder", 4)?;
        state.serialize_field(
            "name",
            &(self.name.clone().into_string().ok().unwrap_or_default()),
        )?;
        state.serialize_field(
            "path",
            &(self.path.clone().into_string().ok().unwrap_or_default()),
        )?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("symbol", &self.symbol)?;
        state.end()
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: OsString,
    pub extension: OsString,
    pub path: OsString,
    pub symbol: String,
}

impl Serialize for File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("File", 3)?;
        state.serialize_field(
            "name",
            &(self.name.clone().into_string().ok().unwrap_or_default()),
        )?;
        state.serialize_field(
            "extension",
            &(self
                .extension
                .clone()
                .into_string()
                .ok()
                .unwrap_or_default()),
        )?;
        state.serialize_field(
            "path",
            &(self.path.clone().into_string().ok().unwrap_or_default()),
        )?;
        state.serialize_field("symbol", &self.symbol);
        state.end()
    }
}
