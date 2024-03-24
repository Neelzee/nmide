use std::{fs::File, path::Path};

use eyre::{Context, Result};

#[test]
fn test_wsfile_creation() -> Result<()> {
    let path = Path::new("src/main.rs").to_owned();
    let file = File::open(path.clone())?;

    Ok(())
}
