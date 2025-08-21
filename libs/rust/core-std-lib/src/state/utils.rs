use crate::{
    html::Html,
    state::{HHMap, Value},
};
use ordered_float::NotNan;
use std::collections::HashMap;

impl Value {
    /// Adds the given value onto itself.
    /// If Value is not an Object variant, its transformed into one,
    /// where `0` is the index of the original value.
    ///
    /// ```rust
    /// use core_std_lib::state::Value;
    /// let value = Value::Null;
    /// let new_val = value.clone().add("foo", Value::Int(0));
    /// assert!(new_val.is_obj());
    /// assert!(new_val.clone().get("foo").unwrap().is_int());
    /// assert_eq!(new_val.get("0").unwrap(), value);
    /// ```
    pub fn add<S: Into<String>>(self, field: S, value: Self) -> Self {
        match self {
            Value::Obj(map) => {
                let mut map = map.to_hm();
                map.insert(field.into(), value);
                Value::Obj(HHMap::from(map))
            }
            _ => Value::new_obj().add("0", self).add(field.into(), value),
        }
    }

    pub fn get<S: Into<String>>(self, field: S) -> Option<Self> {
        match self {
            Value::Obj(map) => map.to_hm().get(&field.into()).cloned(),
            _ => None,
        }
    }

    pub fn remove<S: Into<String>>(self, field: S) -> Self {
        let s: String = field.into();
        let ind = s.parse::<usize>();
        match self {
            Value::Str(mut str) if ind.is_ok() => {
                let ind = ind.unwrap();
                str.remove(ind);
                Value::Str(str)
            }
            Value::List(mut lst) if ind.is_ok() => {
                let ind = ind.unwrap();
                lst.remove(ind);
                Value::List(lst)
            }
            Value::Obj(map) => {
                let mut map = map.to_hm();
                map.remove(&s);
                Value::Obj(HHMap::from(map))
            }
            _ => self,
        }
    }

    pub fn new_str<S: Into<String>>(s: S) -> Self {
        Self::Str(s.into())
    }

    pub fn new_float(f: f32) -> Self {
        Self::Float(NotNan::new(f).unwrap())
    }

    pub fn bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn html(&self) -> Option<Html> {
        match self {
            Self::Html(h) => Some(h.clone()),
            _ => None,
        }
    }

    pub fn is_html(&self) -> bool {
        matches!(self, Self::Html(..))
    }

    pub fn list(&self) -> Option<Vec<Value>> {
        match self {
            Self::List(l) => Some(l.clone()),
            _ => None,
        }
    }

    pub fn new_obj() -> Self {
        Self::Obj(HHMap::default())
    }

    pub fn null(self) -> Option<()> {
        match self {
            Self::Null => Some(()),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    pub fn int(&self) -> Option<i32> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn str(&self) -> Option<String> {
        match self {
            Self::Str(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn is_obj(&self) -> bool {
        matches!(self, Self::Obj(_))
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }

    pub fn obj(&self) -> Option<HashMap<String, Value>> {
        match self {
            Self::Obj(x) => Some(x.clone().to_hm()),
            _ => None,
        }
    }

    pub fn obj_add<S: ToString>(self, field: S, value: Self) -> Self {
        match &self {
            Self::Obj(mp) => {
                let mut mp = mp.clone();
                mp.0.insert(field.to_string(), value);
                Self::Obj(mp)
            }
            _ => self,
        }
    }
}
