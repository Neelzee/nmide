use super::{
    rmap::{RKeyPair, RMap, RValKind, RValue, RValueUnion},
    tmap::{TMap, TValue},
};
use abi_stable::std_types::{RString, RVec};
use std::mem::ManuallyDrop;
impl std::fmt::Debug for RValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RValue")
            .field("kind", &self.kind)
            .field(
                "val",
                match self.kind {
                    RValKind::Int => unsafe { &self.val._int },
                    RValKind::Float => unsafe { &self.val._float },
                    RValKind::Bool => unsafe { &self.val._bool },
                    RValKind::Str => unsafe { &self.val._str },
                    RValKind::List => unsafe { &self.val._lst },
                    RValKind::Obj => unsafe { &self.val._obj },
                },
            )
            .finish()
    }
}

impl PartialEq for RValue {
    fn eq(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (RValKind::Int, RValKind::Int) => unsafe { self.val._int.eq(&self.val._int) },
            (RValKind::Float, RValKind::Float) => unsafe { self.val._float.eq(&self.val._float) },
            (RValKind::Bool, RValKind::Bool) => unsafe { self.val._bool.eq(&self.val._bool) },
            (RValKind::Str, RValKind::Str) => unsafe { self.val._str.eq(&self.val._str) },
            (RValKind::List, RValKind::List) => unsafe { self.val._lst.eq(&self.val._lst) },
            (RValKind::Obj, RValKind::Obj) => unsafe { self.val._obj.eq(&self.val._obj) },
            _ => false,
        }
    }
}

impl Eq for RValue {}

impl Clone for RValue {
    fn clone(&self) -> Self {
        match self.kind {
            RValKind::Int => Self::new_int(self.int().unwrap()),
            RValKind::Float => Self::new_float(self.float().unwrap()),
            RValKind::Bool => Self::new_bool(self.bool().unwrap()),
            RValKind::Str => Self::new_str(self.str().unwrap().to_string()),
            RValKind::List => {
                Self::new_listr(ManuallyDrop::into_inner(self.lst().unwrap().clone()))
            }
            RValKind::Obj => Self {
                kind: RValKind::Obj,
                val: RValueUnion {
                    _obj: self.obj().unwrap().clone(),
                },
            },
        }
    }
}

impl From<i32> for RValue {
    fn from(value: i32) -> Self {
        Self {
            kind: RValKind::Int,
            val: RValueUnion::int(value),
        }
    }
}

impl From<f32> for RValue {
    fn from(value: f32) -> Self {
        Self {
            kind: RValKind::Float,
            val: RValueUnion::float(value),
        }
    }
}

impl From<bool> for RValue {
    fn from(value: bool) -> Self {
        Self {
            kind: RValKind::Bool,
            val: RValueUnion::bool(value),
        }
    }
}

impl From<String> for RValue {
    fn from(value: String) -> Self {
        Self {
            kind: RValKind::Str,
            val: RValueUnion::str(value),
        }
    }
}

impl From<&str> for RValue {
    fn from(value: &str) -> Self {
        Self {
            kind: RValKind::Str,
            val: RValueUnion::str(value),
        }
    }
}

impl<T: Into<RValue>> From<RVec<T>> for RValue {
    fn from(value: RVec<T>) -> Self {
        Self {
            kind: RValKind::List,
            val: RValueUnion::listr(value),
        }
    }
}

impl<T: Into<RValue>> From<Vec<T>> for RValue {
    fn from(value: Vec<T>) -> Self {
        Self {
            kind: RValKind::List,
            val: RValueUnion::list(value),
        }
    }
}

impl<S: ToString, T: Into<RValue>> From<Vec<(S, T)>> for RValue {
    fn from(value: Vec<(S, T)>) -> Self {
        Self {
            kind: RValKind::Obj,
            val: RValueUnion::obj(value),
        }
    }
}

impl From<TValue> for RValue {
    fn from(value: TValue) -> Self {
        match value {
            TValue::Int(i) => Self {
                kind: RValKind::Int,
                val: RValueUnion::int(i),
            },
            TValue::Float(f) => Self {
                kind: RValKind::Float,
                val: RValueUnion::float(f),
            },
            TValue::Bool(b) => Self {
                kind: RValKind::Bool,
                val: RValueUnion::bool(b),
            },
            TValue::Str(s) => Self {
                kind: RValKind::Str,
                val: RValueUnion::str(s),
            },
            TValue::List(l) => Self {
                kind: RValKind::List,
                val: RValueUnion::list(l),
            },
            TValue::Obj(o) => Self {
                kind: RValKind::Obj,
                val: RValueUnion::obj(o),
            },
        }
    }
}

impl<S: ToString, T: Into<RValue>> From<(S, T)> for RKeyPair {
    fn from(value: (S, T)) -> Self {
        let (s, val) = value;
        let mut rstr = RString::new();
        rstr.push_str(s.to_string().as_str());
        Self {
            key: rstr,
            val: val.into(),
        }
    }
}

impl Default for RMap {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TMap> for RMap {
    fn from(value: TMap) -> Self {
        Self {
            pairs: value.0.into_iter().map(|t| t.into()).collect(),
        }
    }
}