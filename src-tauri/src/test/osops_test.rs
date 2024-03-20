use crate::osops::get_paths;
use eyre::{Context, Result};
use std::path::Path;

#[test]
fn test_path_getter() -> Result<()> {
    let path = Path::new("./src/test");

    let dirs = get_paths(path, 1)?;

    Ok(())
}
