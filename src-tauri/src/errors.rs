use crate::{either::Either, nmrep};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NmideError<T> {
    pub val: T,
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
    pub fn push_nmide(mut self, other: NmideReport) -> Self {
        if self.rep.is_none() {
            self.rep = Some(other);
        } else {
            self.rep.and_then(|mut r| Some(r.stack.push(other)));
        }

        self
    }

    /// Applies F to the val, pushing the NmideReport to the stack if it fails.
    pub fn or_else<F, U>(&self, f: F) -> NmideError<U>
    where
        F: FnOnce(T) -> NmideError<U>,
    {
        let (val, rep) = f(self.val).unwrap_with_err();
        NmideError {
            val,
            rep: nmrep!(self.rep, rep),
        }
    }

    /// Attempts to unwrap NmideError
    ///
    /// If there are any NmideReport's, returns them, else val
    pub fn unwrap_or_err(self) -> Result<T, NmideReport> {
        match self.rep {
            Some(err) => Err(err),
            None => Ok(self.val),
        }
    }

    /// Ignores errors
    pub fn ignore_err(self) -> T {
        self.val
    }

    pub fn unwrap_with_err(self) -> (T, Option<NmideReport>) {
        (self.val, self.rep)
    }

    /// Applies the function to self, carrying with the reports
    pub fn map<F, U>(self, f: F) -> NmideError<U>
    where
        F: FnOnce(Self) -> NmideError<U>,
    {
        let res = f(self);

        NmideError {
            val: res.val,
            rep: nmrep!(res.rep, self.rep),
        }
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

impl<T> NmideError<Option<T>> {
    pub fn transpose(self) -> Option<NmideError<T>> {
        if self.val.is_none() {
            return None;
        } else {
            return Some(NmideError {
                val: self.val.unwrap(),
                rep: self.rep,
            });
        }
    }
}

impl<T, E: std::error::Error> NmideError<Result<T, E>> {
    pub fn transpose(self) -> Result<T, NmideReport> {
        match self.val {
            Ok(v) => Ok(v),
            Err(err) => {
                self.push_nmide(NmideReport {
                    msg: format!(
                        "Cause: `{:?}`, Description: ´{:?}´",
                        err.source(),
                        err.to_string()
                    ),
                    lvl: ErrorLevel::Unknown,
                    tag: Vec::new(),
                    stack: Vec::new(),
                    origin: format!("{:?}", err.source()),
                });

                Err(self.rep.unwrap())
            }
        }
    }

    pub fn transform(self) -> NmideError<Option<T>> {
        match self.val {
            Ok(v) => NmideError {
                val: Some(v),
                rep: self.rep,
            },
            Err(err) => {
                self.push_nmide(NmideReport {
                    msg: format!(
                        "Cause: `{:?}`, Description: ´{:?}´",
                        err.source(),
                        err.to_string()
                    ),
                    lvl: ErrorLevel::Unknown,
                    tag: Vec::new(),
                    stack: Vec::new(),
                    origin: format!("{:?}", err.source()),
                });

                NmideError {
                    val: None,
                    rep: self.rep,
                }
            }
        }
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
