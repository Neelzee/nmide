use eyre::ErrReport;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct NmideError {
    pub msg: String,
    pub lvl: ErrorLevel,
    pub tag: Vec<ErrorTag>,
    pub stack: Vec<NmideError>,
    pub origin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorLevel {
    Unknown,
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorTag {
    WSError,
    NonSpecified,
}

impl Display for ErrorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorLevel::Unknown => write!(f, "Unknown"),
            ErrorLevel::Low => write!(f, "Low"),
            ErrorLevel::Medium => write!(f, "Medium"),
            ErrorLevel::High => write!(f, "High"),
        }
    }
}

impl From<ErrReport> for NmideError {
    fn from(value: ErrReport) -> Self {
        NmideError {
            msg: value.to_string(),
            lvl: ErrorLevel::Unknown,
            tag: Vec::new(),
            stack: value
                .chain()
                .into_iter()
                .map(|e| NmideError {
                    msg: e.to_string(),
                    lvl: ErrorLevel::Unknown,
                    tag: Vec::new(),
                    stack: Vec::new(),
                    origin: e
                        .source()
                        .and_then(|err| Some(err.to_string()))
                        .unwrap_or_default(),
                })
                .collect(),
            origin: value
                .source()
                .and_then(|err| Some(err.to_string()))
                .unwrap_or_default(),
        }
    }
}

impl Display for NmideError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WSError: `{}`, level: `{}`", self.msg, self.lvl)
    }
}

pub fn set_lvl(err: ErrReport, lvl: ErrorLevel) -> NmideError {
    let nerr: NmideError = err.into();
    NmideError { lvl, ..nerr }
}
