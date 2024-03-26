use crate::{either::Either, nmrep};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NmideError<T> {
    pub val: T,
    pub rep: Option<NmideReport>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ErrorLevel {
    #[default]
    Unknown,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn push_nmide(self, other: NmideReport) -> Self {
        let (val, rep) = self.unwrap_with_err();
        if let Some(r) = rep {
            NmideError {
                val,
                rep: Some(r.push_stack(Some(other))),
            }
        } else {
            NmideError {
                val,
                rep: Some(other),
            }
        }
    }

    pub fn from_err<E>(val: T, err: E) -> Self
    where
        E: std::error::Error,
    {
        NmideError {
            val,
            rep: Some(NmideReport {
                msg: format!("{err:?}"),
                lvl: ErrorLevel::Unknown,
                tag: Vec::new(),
                stack: Vec::new(),
                origin: format!("{:?}", err.source()),
            }),
        }
    }

    /// Applies F to the val, pushing the NmideReport to the stack if it fails.
    pub fn or_else<F, U>(self, f: F) -> NmideError<U>
    where
        F: FnOnce(T) -> NmideError<U>,
    {
        let (v, r) = self.unwrap_with_err();
        let (val, rep) = f(v).unwrap_with_err();
        NmideError {
            val,
            rep: nmrep!(r, rep),
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
        let r = self.rep.clone();
        let res = f(self);

        NmideError {
            val: res.val,
            rep: nmrep!(res.rep, r),
        }
    }

    /// Applies a function on the value
    pub fn vmap<F, U>(self, f: F) -> NmideError<U>
    where
        F: FnOnce(T) -> U,
    {
        NmideError {
            val: f(self.val),
            rep: self.rep,
        }
    }
}

impl<T> Display for NmideError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WSError: `{:?}`, level: `{:?}`",
            self.rep.clone().and_then(|r| Some(r.msg)),
            self.rep.clone().and_then(|r| Some(r.lvl))
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
        let (val, rep) = self.unwrap_with_err();
        match val {
            Ok(v) => Ok(v),
            Err(err) => {
                let nm = NmideReport {
                    msg: format!(
                        "Cause: `{:?}`, Description: ´{:?}´",
                        err.source(),
                        err.to_string()
                    ),
                    lvl: ErrorLevel::Unknown,
                    tag: Vec::new(),
                    stack: Vec::new(),
                    origin: format!("{:?}", err.source()),
                };
                if let Some(r) = rep {
                    return Err(r.push_stack(Some(nm)));
                } else {
                    return Err(nm);
                }
            }
        }
    }

    pub fn transform(self) -> NmideError<Option<T>> {
        let mut rep = self.rep;
        match self.val {
            Ok(v) => NmideError { val: Some(v), rep },
            Err(err) => {
                if let Some(r) = rep {
                    rep = Some(r.push_stack(Some(NmideReport {
                        msg: format!(
                            "Cause: `{:?}`, Description: ´{:?}´",
                            err.source(),
                            err.to_string()
                        ),
                        lvl: ErrorLevel::Unknown,
                        tag: Vec::new(),
                        stack: Vec::new(),
                        origin: format!("{:?}", err.source()),
                    })));
                }
                NmideError { val: None, rep }
            }
        }
    }
}

pub fn collect<T>(vec: Vec<NmideError<T>>) -> (Vec<T>, Option<NmideReport>) {
    vec.into_iter().map(|e| e.unwrap_with_err()).fold(
        (Vec::new(), None::<NmideReport>),
        |(mut vals, err), (v, e)| {
            vals.push(v);
            match (err, e) {
                (Some(a), b) => (vals, Some(a.push_stack(b))),
                (_, b) => (vals, b),
            }
        },
    )
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
