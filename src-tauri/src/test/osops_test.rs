use crate::osops::get_paths;
use eyre::{Context, Result};
use log::debug;
use std::path::Path;

#[test]
fn test_path_getter() -> Result<()> {
    let path = Path::new("./src/test");

    let (dirs, dirs_rep) = get_paths(path, 1).unwrap_with_err();

    debug!("{dirs:?}");
    debug!("\n{dirs_rep:?}");

    assert!(dirs.len() != 0);

    Ok(())
}
