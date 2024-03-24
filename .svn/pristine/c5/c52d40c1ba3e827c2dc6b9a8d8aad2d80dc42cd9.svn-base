use std::fmt::Display;

use eyre::Error;

#[derive(Debug)]
pub enum NmideError {
    OptionToResult(String),
}

impl Display for NmideError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NmideError::OptionToResult(s) => write!(f, "OptionToResultError: {}", s),
        }
    }
}
