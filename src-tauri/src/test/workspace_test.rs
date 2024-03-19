use std::{fs::File, path::Path};

use eyre::{Context, Result};

use crate::workspace::ws_file::WSFile;

#[test]
fn test_wsfile_creation() -> Result<()> {
    let path = Path::new("src/main.rs");
    let file = File::open(path)?;

    let wsfile = WSFile::new(path, &file);

    assert!(wsfile.is_ok());

    Ok(())
}
