use crate::types::Fof;
use eyre::{Context, Result};
use std::{
    io::{BufWriter, Write},
    path::Path,
};

pub fn get_files(path: String) -> Result<Fof> {
    Ok(Fof::Folder(crate::types::Folder::new(Path::new(&path))?))
}

pub fn save_file(path: &Path, content: String) -> Result<()> {
    let file = std::fs::File::open(path)?;
    let mut writer = BufWriter::new(file);

    writer
        .write_all(content.as_bytes())
        .wrap_err("Failed saving content to file")
}
