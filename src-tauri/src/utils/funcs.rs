use std::ffi::OsStr;

use eyre::{eyre, Context, Result};

use crate::errors::NmideError;

pub fn os_to_str(s: &OsStr) -> Result<String> {
    Ok(s.to_str()
        .ok_or(eyre!(NmideError::OptionToResult(format!("`{s:?}`"))))?
        .to_string())
}
