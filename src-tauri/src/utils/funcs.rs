use std::ffi::OsStr;

use eyre::{eyre, Context, Result};

use crate::errors::NmideError;

pub fn os_to_str(s: &OsStr) -> Result<String> {
    Ok(s.to_str()
        .ok_or(eyre!(NmideError {
            msg: format!("Failed converting String: `{s:?}`"),
            lvl: crate::errors::ErrorLevel::Low,
            tag: Vec::new()
        }))?
        .to_string())
}
