use std::ffi::OsStr;

use eyre::{Context, Result};

pub fn os_to_str(s: &OsStr) -> Result<String> {
    Ok(s.to_str()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed converting from OsStr to String",
        ))
        .wrap_err("failed")?
        .to_string())
}
