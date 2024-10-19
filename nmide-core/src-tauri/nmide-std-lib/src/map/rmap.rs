use abi_stable::{
    std_types::{ROption, RString, RVec},
    StableAbi,
};
use std::mem::ManuallyDrop;

use super::tmap::{TMap, TValue};

#[repr(C)]
#[derive(StableAbi)]
pub struct RValue {
    pub(crate) kind: RValKind,
    pub(crate) val: RValueUnion,
}

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

impl RValue {
    pub fn new_int(i: i32) -> Self {
        i.into()
    }

    pub fn new_float(f: f32) -> Self {
        f.into()
    }

    pub fn new_bool(b: bool) -> Self {
        b.into()
    }

    pub fn new_str<S: ToString>(s: S) -> Self {
        s.to_string().into()
    }

    pub fn new_list<T: Into<RValue>>(list: Vec<T>) -> Self {
        list.into()
    }

    pub fn new_listr<T: Into<RValue>>(list: RVec<T>) -> Self {
        list.into()
    }

    pub fn new_obj<S: ToString, T: Into<RValue>>(list: Vec<(S, T)>) -> Self {
        list.into()
    }

    pub fn int(&self) -> Option<i32> {
        if self.kind == RValKind::Int {
            Some(unsafe { self.val._int })
        } else {
            None
        }
    }

    pub fn float(&self) -> Option<f32> {
        if self.kind == RValKind::Float {
            Some(unsafe { self.val._float })
        } else {
            None
        }
    }

    pub fn bool(&self) -> Option<bool> {
        if self.kind == RValKind::Bool {
            Some(unsafe { self.val._bool })
        } else {
            None
        }
    }

    pub fn str(&self) -> Option<&ManuallyDrop<RString>> {
        if self.kind == RValKind::Str {
            Some(unsafe { &self.val._str })
        } else {
            None
        }
    }

    pub fn lst(&self) -> Option<&ManuallyDrop<RVec<RValue>>> {
        if self.kind == RValKind::List {
            Some(unsafe { &self.val._lst })
        } else {
            None
        }
    }

    pub fn obj(&self) -> Option<&ManuallyDrop<RVec<RKeyPair>>> {
        if self.kind == RValKind::Obj {
            Some(unsafe { &self.val._obj })
        } else {
            None
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

#[repr(C)]
#[derive(StableAbi)]
pub union RValueUnion {
    _int: i32,
    _float: f32,
    _bool: bool,
    _str: ManuallyDrop<RString>,
    _lst: ManuallyDrop<RVec<RValue>>,
    _obj: ManuallyDrop<RVec<RKeyPair>>,
}

impl RValueUnion {
    pub fn int(_int: i32) -> Self {
        Self { _int }
    }

    pub fn float(_float: f32) -> Self {
        Self { _float }
    }

    pub fn bool(_bool: bool) -> Self {
        Self { _bool }
    }

    pub fn str<S>(s: S) -> Self
    where
        S: ToString,
    {
        let mut rstr = RString::new();
        rstr.push_str(s.to_string().as_str());
        Self {
            _str: ManuallyDrop::new(rstr),
        }
    }

    pub fn list<T>(lst: Vec<T>) -> Self
    where
        T: Into<RValue>,
    {
        Self {
            _lst: ManuallyDrop::new(RVec::from_iter(lst.into_iter().map(|t| t.into()))),
        }
    }

    pub fn listr<T>(lst: RVec<T>) -> Self
    where
        T: Into<RValue>,
    {
        Self {
            _lst: ManuallyDrop::new(lst.into_iter().map(|v| v.into()).collect()),
        }
    }

    pub fn obj<S, T>(lst: Vec<(S, T)>) -> Self
    where
        S: ToString,
        T: Into<RValue>,
    {
        Self {
            _obj: ManuallyDrop::new(RVec::from_iter(lst.into_iter().map(|t| t.into()))),
        }
    }

    pub fn objr<S, T>(lst: RVec<(S, T)>) -> Self
    where
        S: ToString,
        T: Into<RValue>,
    {
        Self {
            _obj: ManuallyDrop::new(lst.into_iter().map(|t| t.into()).collect()),
        }
    }
}

#[repr(C)]
#[derive(StableAbi, Clone, PartialEq, Eq, Debug)]
pub struct RKeyPair {
    pub(crate) key: RString,
    pub(crate) val: RValue,
}

impl RKeyPair {
    pub fn new<K: ToString, V: Into<RValue>>(key: K, val: V) -> Self {
        let mut rstr = RString::new();
        rstr.push_str(&key.to_string().to_lowercase());

        Self {
            key: rstr,
            val: val.into(),
        }
    }

    /// Returns true if the given string is equal to the key.
    ///
    /// ```rust
    /// use nmide_std_lib::map::rmap::RKeyPair;
    /// let key = String::from("foo");
    /// let other_key = "bar";
    /// assert!(
    ///     RKeyPair::new(&key, 1).cmp_key(&key),
    ///     "keypair contains key foo"
    /// );
    /// assert!(
    ///     !RKeyPair::new(&key, 1).cmp_key(&other_key),
    ///     "keypair does not contain key bar"
    /// );
    /// ```
    pub fn cmp_key<S>(&self, key: &S) -> bool
    where
        S: ToString + ?Sized,
    {
        self.key
            .clone()
            .to_string()
            .eq_ignore_ascii_case(&key.to_string())
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

#[repr(u8)]
#[derive(StableAbi, Clone, PartialEq, Eq, Debug)]
pub enum RValKind {
    Int,
    Float,
    Bool,
    Str,
    List,
    Obj,
}

#[repr(C)]
#[derive(StableAbi, Clone, PartialEq, Eq, Debug)]
pub struct RMap {
    pub(crate) pairs: RVec<RKeyPair>,
}

impl RMap {
    pub fn new() -> RMap {
        RMap { pairs: RVec::new() }
    }

    /// Returns if the given key is in the map.
    /// Only checks the first-level.
    ///
    /// ```rust
    /// use nmide_std_lib::map::rmap::RMap;
    /// let key = String::from("foo");
    /// assert!(RMap::new().insert(&key, 0).contains_key(&key));
    /// assert!(!RMap::new().contains_key(&key));
    /// ```
    pub fn contains_key<S>(&self, key: &S) -> bool
    where
        S: ToString + ?Sized,
    {
        self.pairs.iter().any(|kp| kp.cmp_key(key))
    }

    /// Merges self with other map, if other has same fields, they are
    /// overwritten with values from self.
    ///
    /// For a set of all maps, MAPS, this monoid holds:
    /// (Map::merge, MAPS, Map::new)
    ///
    /// ```rust
    /// use nmide_std_lib::map::rmap::RMap;
    /// let key = String::from("foo");
    /// assert_eq!(RMap::new().merge(RMap::new()), RMap::new());
    /// assert_eq!(
    ///     RMap::new().merge(RMap::new().insert(&key, 0)),
    ///     RMap::new().insert(&key, 0)
    /// );
    /// assert_eq!(
    ///     RMap::new().insert(&key, 0).merge(RMap::new()),
    ///     RMap::new().insert(&key, 0)
    /// );
    /// ```
    pub fn merge(self, other: Self) -> Self {
        let mut pairs = self.pairs.clone();
        let mut other_pairs = other
            .pairs
            .into_iter()
            .filter(move |pk| !self.contains_key(&pk.key))
            .collect();
        pairs.append(&mut other_pairs);
        Self { pairs }
    }

    /// Inserts the given value to the given key.
    /// If it already exists in the map, updates the value instead.
    ///
    /// ```rust
    /// use nmide_std_lib::map::rmap::RMap;
    /// let key = String::from("foo");
    /// let other_key = String::from("foobar");
    /// let mut a = RMap::new();
    /// a.insert_mut(&key, 1);
    /// let mut b = RMap::new();
    /// b.insert_mut(&key, 1);
    /// assert_eq!(a, b);
    /// assert!(a.lookup(&key).is_some());
    /// assert!(a.lookup(&other_key).is_none());
    /// ```
    pub fn insert_mut<S, T>(&mut self, key: &S, val: T)
    where
        S: ToString + ?Sized + std::fmt::Display,
        T: Into<RValue> + Clone,
    {
        if self.contains_key(key) {
            self.pairs = self
                .pairs
                .iter()
                .filter(|kp| kp.cmp_key(key))
                .cloned()
                .collect::<RVec<_>>();
        }
        self.pairs.push((key, val).into());
    }

    /// Inserts the given value to the given key.
    /// If it already exists in the map, updates the value instead.
    ///
    /// ```rust
    /// use nmide_std_lib::map::rmap::RMap;
    /// let key = String::from("foo");
    /// let other_key = String::from("foobar");
    /// assert_eq!(RMap::new().insert(&key, 1), RMap::new().insert(&key, 1));
    /// assert!(RMap::new().insert(&key, 1).lookup(&key).is_some());
    /// assert!(RMap::new().insert(&key, 1).lookup(&other_key).is_none());
    /// ```
    pub fn insert<S, T>(self, key: &S, val: T) -> Self
    where
        S: ToString + std::fmt::Display + ?Sized,
        T: Into<RValue> + Clone,
    {
        Self {
            pairs: if self.contains_key(key) {
                self.pairs
                    .into_iter()
                    .map(|mut pair| {
                        if pair.cmp_key(key) {
                            pair.val = val.clone().into();
                            return pair;
                        }

                        pair
                    })
                    .collect()
            } else {
                let mut pairs = self.pairs;
                pairs.push((key, val).into());
                pairs
            },
        }
    }

    /// Checks the map for the given key, if it exists, returns the value. If
    /// it doesn't exist, returns none.
    ///
    /// ```rust
    /// use nmide_std_lib::map::rmap::RMap;
    /// let key = String::from("foo");
    /// let other_key = String::from("foobar");
    /// assert!(RMap::new().insert(&key, 1).lookup(&key).is_some());
    /// assert!(RMap::new().insert(&key, 1).lookup(&other_key).is_none());
    /// ```
    pub fn lookup<S>(&self, key: &S) -> ROption<&RValue>
    where
        S: ToString + ?Sized,
    {
        for p in self.pairs.iter() {
            if p.cmp_key(key) {
                return ROption::RSome(&p.val);
            }
        }
        ROption::RNone
    }

    /// Removes the given value, by the given key, returning it if it exists.
    ///
    /// ```rust
    /// use nmide_std_lib::map::rmap::RMap;
    /// let key = String::from("foo");
    /// let mut map = RMap::new().insert(&key, 1);
    /// assert!(map.lookup(&key).is_some());
    /// assert!(map.remove_mut(&key).is_some());
    /// assert!(RMap::new().remove_mut(&key).is_none());
    /// ```
    pub fn remove_mut<S>(&mut self, key: &S) -> ROption<RValue>
    where
        S: ToString,
    {
        for (index, p) in self.pairs.iter().enumerate() {
            if p.cmp_key(key) {
                return ROption::RSome(self.pairs.swap_remove(index).val);
            }
        }
        ROption::RNone
    }

    /// Removes the given value, by the given key.
    ///
    /// ```rust
    /// use nmide_std_lib::map::rmap::RMap;
    /// let key = String::from("foo");
    /// let mut map = RMap::new().insert(&key, 1);
    /// assert!(map.lookup(&key).is_some());
    /// assert!(map.remove(&key).lookup(&key).is_none());
    /// assert!(RMap::new().remove(&key).lookup(&key).is_none());
    /// ```
    pub fn remove<S>(self, key: &S) -> Self
    where
        S: ToString,
    {
        Self {
            pairs: self.pairs.into_iter().filter(|p| !p.cmp_key(key)).collect(),
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
