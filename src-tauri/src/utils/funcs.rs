use std::ffi::OsStr;

use crate::errors::{ErrorLevel, NmideError, NmideReport};

pub fn os_to_str(s: &OsStr) -> NmideError<String> {
    NmideError {
        val: s
            .to_str()
            .and_then(|s| Some(s.to_string()))
            .unwrap_or(format!("{s:?}")),
        rep: Some(NmideReport {
            msg: format!("Failed converting String: `{s:?}`"),
            lvl: ErrorLevel::Low,
            tag: Vec::new(),
            stack: Vec::new(),
            origin: "os_to_str".to_string(),
        }),
    }
}
