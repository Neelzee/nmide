use std::ffi::OsStr;

use eyre::{eyre, Result};

use crate::errors::NmideError;

pub fn os_to_str(s: &OsStr) -> Result<String> {
    Ok(s.to_str()
        .ok_or(eyre!(NmideError {
            msg: format!("Failed converting String: `{s:?}`"),
            lvl: crate::errors::ErrorLevel::Low,
            tag: Vec::new(),
            stack: Vec::new(),
            origin: "os_to_str".to_string(),
        }))?
        .to_string())
}
