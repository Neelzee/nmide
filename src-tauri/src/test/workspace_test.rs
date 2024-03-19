use std::{fs::File, path::Path};

use eyre::{Context, Result};

use crate::workspace::ws_file::WSFile;

#[test]
fn test_wsfile_creation() -> Result<()> {
    let path = Path::new("src/main.rs").to_owned();
    let file = File::open(path.clone())?;

    let wsfile = WSFile::new(path, Box::new(file));

    assert!(wsfile.is_ok());

    Ok(())
}
