use crate::nmrep;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NmideError<T> {
    pub val: T,
    pub rep: Option<NmideReport>,
}

impl<T: Clone> Clone for NmideError<T> {
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
            rep: self.rep.clone(),
        }
    }
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
    pub fn new<S>(msg: S, origin: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            msg: msg.into(),
            lvl: ErrorLevel::Unknown,
            tag: Vec::new(),
            stack: Vec::new(),
            origin: origin.into(),
        }
    }

    pub fn push_stack(self, other: Option<NmideReport>) -> Self {
        let mut stack = self.stack;
        if let Some(o) = other {
            stack.push(o);
        }
        Self { stack, ..self }
    }

    pub fn from_res<T, E>(err: Result<T, E>) -> Option<Self>
    where
        E: std::error::Error,
    {
        err.err().map(|e| NmideReport {
            msg: format!("{e:?}"),
            lvl: ErrorLevel::Unknown,
            tag: Vec::new(),
            stack: Vec::new(),
            origin: format!("{:?}", e.source()),
        })
    }

    pub fn from_err<E>(err: E) -> Self
    where
        E: std::error::Error,
    {
        NmideReport {
            msg: format!("{err:?}"),
            lvl: ErrorLevel::Unknown,
            tag: Vec::new(),
            stack: Vec::new(),
            origin: format!("{:?}", err.source()),
        }
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
    pub fn new(val: T) -> Self {
        Self { val, rep: None }
    }

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

    pub fn from_err<E>(err: Result<T, E>) -> NmideError<Option<T>>
    where
        E: std::error::Error,
    {
        match err {
            Ok(t) => NmideError {
                val: Some(t),
                rep: None,
            },
            Err(e) => NmideError {
                val: None,
                rep: Some(NmideReport {
                    msg: format!("{e:?}"),
                    lvl: ErrorLevel::Unknown,
                    tag: Vec::new(),
                    stack: Vec::new(),
                    origin: format!("{:?}", e.source()),
                }),
            },
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
        F: FnOnce(T) -> NmideError<U>,
    {
        let (v, rep) = self.unwrap_with_err();
        let (val, r) = f(v).unwrap_with_err();

        NmideError {
            val,
            rep: nmrep!(rep, r),
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
            self.rep.clone().map(|r| r.msg),
            self.rep.clone().map(|r| r.lvl)
        )
    }
}

impl<T> NmideError<Option<T>> {
    pub fn transpose(self) -> Option<NmideError<T>> {
        if self.val.is_none() {
            None
        } else {
            Some(NmideError {
                val: self.val.unwrap(),
                rep: self.rep,
            })
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
                    Err(r.push_stack(Some(nm)))
                } else {
                    Err(nm)
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

impl<T> NmideError<NmideError<T>> {
    pub fn fold(self) -> NmideError<T> {
        let (v, r) = self.unwrap_with_err();

        if let Some(rep) = r {
            let e = v.push_nmide(rep);
            NmideError {
                val: e.val,
                rep: e.rep,
            }
        } else {
            NmideError {
                val: v.val,
                rep: v.rep,
            }
        }
    }
}

impl<T> NmideError<Option<NmideError<T>>> {
    pub fn option_combine(self) -> NmideError<Option<T>> {
        let (val, rep) = self.unwrap_with_err();

        match val {
            Some(v) => {
                let n = v.vmap(|t| Some(t));
                if let Some(rep) = rep {
                    n.push_nmide(rep)
                } else {
                    n
                }
            }
            None => NmideError {
                val: None::<T>,
                rep,
            },
        }
    }
}

pub fn filter_nmide<T>(vec: Vec<NmideError<T>>) -> (Vec<T>, Option<NmideReport>) {
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

pub fn fold_nmide<T>(vec: Vec<NmideError<T>>) -> NmideError<Vec<T>> {
    vec.into_iter().fold(
        NmideError {
            val: Vec::new(),
            rep: None,
        },
        |mut err, e| {
            err.val.push(e.val);
            if let Some(r) = e.rep {
                err = err.push_nmide(r);
            }
            err
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
                    (Some(a), b) => Some(a.push_stack(b)),
                })
        }
    };
}

#[macro_export]
macro_rules! nmfold {
    ($vec:expr) => {{
        $vec.into_iter().fold(
            NmideError {
                val: Vec::new(),
                rep: None,
            },
            |mut err, e| {
                err.val.push(e.val);
                if let Some(rep) = e.rep {
                    err = err.push_nmide(rep);
                }
                err
            },
        )
    }};
}
