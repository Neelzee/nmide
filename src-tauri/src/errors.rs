use crate::{either::Either, nmrep};
use either::Either;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NmideError<T> {
    pub val: Option<T>,
    pub rep: Option<NmideReport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NmideReport {
    pub msg: String,
    pub lvl: ErrorLevel,
    pub tag: Vec<ErrorTag>,
    pub stack: Vec<NmideReport>,
    pub origin: String,
}

impl NmideReport {
    pub fn push_stack(self, other: Option<NmideReport>) -> Self {
        let mut stack = self.stack;
        if let Some(o) = other {
            stack.push(o);
        }
        Self { stack, ..self }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ErrorLevel {
    #[default]
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

impl<T> NmideError<T> {
    pub fn empty() -> Self {
        Self {
            val: None,
            rep: None,
        }
    }
    pub fn push_nmide(mut self, other: NmideReport) -> Self {
        if self.rep.is_none() {
            self.rep = Some(other);
        } else {
            self.rep.and_then(|mut r| Some(r.stack.push(other)));
        }

        self
    }

    pub fn and(self, v: Option<T>) -> NmideError<T> {
        let val = self.val.and(v);

        Self { val, rep: self.rep }
    }

    pub fn or<U>(self, err: NmideError<U>) -> NmideError<U> {
        NmideError {
            val: err.val,
            rep: nmrep!(self.rep, err.rep),
        }
    }

    pub fn and_then<F, U>(self, f: F) -> NmideError<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        let val = self.val.and_then(f);
        NmideError { val, rep: self.rep }
    }

    /// Applies F to the val, pushing the NmideReport to the stack if it fails.
    ///
    /// Returns None if val is None
    pub fn or_else<F, U>(&self, f: F) -> Option<NmideError<U>>
    where
        F: FnOnce(T) -> NmideError<U>,
    {
        match self.val {
            Some(v) => {
                let (val, rep) = f(v).unwrap_with_err();
                return Some(NmideError {
                    val,
                    rep: nmrep!(self.rep, rep),
                });
            }
            None => None,
        }
    }

    pub fn is_some(&self) -> bool {
        self.val.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.val.is_none()
    }

    pub fn unwrap(self) -> T {
        self.val.unwrap()
    }

    pub fn unwrap_or(self, val: T) -> T {
        self.val.unwrap_or(val)
    }

    pub fn unwrap_with_err(self) -> (Option<T>, Option<NmideReport>) {
        (self.val, self.rep)
    }
}

impl<T: Default> NmideError<T> {
    pub fn unwrap_or_default(self) -> T {
        self.val.unwrap_or_default()
    }
}

impl<T> Display for NmideError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WSError: `{:?}`, level: `{:?}`",
            self.rep.and_then(|r| Some(r.msg)),
            self.rep.and_then(|r| Some(r.lvl))
        )
    }
}

#[macro_export]
macro_rules! nmrep {
    ($($item:expr),*) => {
        {
            vec![$($item),*]
                .into_iter()
                .fold(None, |a, b| match (a, b) {
                    (None, None) => None,
                    (None, b) => b,
                    (Some(mut a), b) => Some(a.push_stack(b)),
                })
        }
    };
}
