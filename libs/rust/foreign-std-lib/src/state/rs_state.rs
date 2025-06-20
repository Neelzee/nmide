//! RState

// TODO: Add doc-string

use abi_stable::{
    StableAbi,
    std_types::{ROption, RString, RVec},
};
use core_std_lib::state::Value;
use ordered_float::NotNan;
use rstest::rstest;
use std::{collections::HashMap, convert::Into, mem::ManuallyDrop};

use crate::html::rs_html::RHtml;

#[repr(C)]
#[derive(StableAbi)]
pub struct RValue {
    pub(crate) kind: RValKind,
    pub(crate) val: RValueUnion,
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

    // TODO: Add doc-test
    pub fn int(&self) -> Option<i32> {
        if self.kind == RValKind::Int {
            Some(unsafe { self.val._int })
        } else {
            None
        }
    }

    // TODO: Add doc-test
    pub fn float(&self) -> Option<f32> {
        if self.kind == RValKind::Float {
            Some(unsafe { self.val._float })
        } else {
            None
        }
    }

    // TODO: Add doc-test
    pub fn bool(&self) -> Option<bool> {
        if self.kind == RValKind::Bool {
            Some(unsafe { self.val._bool })
        } else {
            None
        }
    }

    // TODO: Add doc-test
    pub fn str(&self) -> Option<&ManuallyDrop<RString>> {
        if self.kind == RValKind::Str {
            Some(unsafe { &self.val._str })
        } else {
            None
        }
    }

    // TODO: Add doc-test
    pub fn lst(&self) -> Option<&ManuallyDrop<RVec<RValue>>> {
        if self.kind == RValKind::List {
            Some(unsafe { &self.val._lst })
        } else {
            None
        }
    }

    // TODO: Add doc-test
    pub fn obj(&self) -> Option<&ManuallyDrop<RVec<RKeyPair>>> {
        if self.kind == RValKind::Obj {
            Some(unsafe { &self.val._obj })
        } else {
            None
        }
    }

    pub fn to_value(self) -> Value {
        match &self.kind {
            RValKind::Int => Value::Int(self.int().unwrap_or_default()),
            RValKind::Float => {
                Value::Float(NotNan::new(self.float().unwrap_or_default()).unwrap_or_default())
            }
            RValKind::Bool => Value::Bool(self.bool().unwrap()),
            RValKind::Str => {
                // TODO: Is this dropped?
                let mn = self
                    .str()
                    .cloned()
                    .unwrap_or(ManuallyDrop::new(RString::new()));
                let s = mn.as_str().to_string();
                Value::Str(s)
            }
            RValKind::List => Value::List(
                self.lst()
                    .cloned()
                    .unwrap_or(ManuallyDrop::new(Default::default()))
                    .iter()
                    .map(|v| v.clone().to_value())
                    .collect(),
            ),
            RValKind::Obj => {
                let xs = self
                    .obj()
                    .cloned()
                    .unwrap_or(ManuallyDrop::new(Default::default()));
                let ys: HashMap<String, Value> =
                    xs.clone().iter().map(|y| y.clone().to_tuple()).collect();
                Value::Obj(ys.into())
            }
            RValKind::Null => Value::Null,
            RValKind::Html => {
                let html = unsafe { self.val._html };
                Value::Html(html.clone().to_html())
            }
        }
    }
}

#[repr(C)]
#[derive(StableAbi)]
pub union RValueUnion {
    pub(crate) _null: bool,
    pub(crate) _int: i32,
    pub(crate) _float: f32,
    pub(crate) _bool: bool,
    pub(crate) _str: ManuallyDrop<RString>,
    pub(crate) _lst: ManuallyDrop<RVec<RValue>>,
    pub(crate) _obj: ManuallyDrop<RVec<RKeyPair>>,
    pub(crate) _html: ManuallyDrop<RHtml>,
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
    /// use foreign_std_lib::state::rs_state::RKeyPair;
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

    pub fn val(&self) -> &RValue {
        &self.val
    }

    pub fn to_tuple(self) -> (String, Value) {
        (self.key.as_str().to_string(), self.val().clone().to_value())
    }
}

#[rstest]
#[case("foo", "bar", "foo", "Comparing 'foo' and 'foo' should be true")]
#[case(1, 1, 1, "Comparing '1' and '1' should be true")]
#[case(
    "fooBAR",
    1,
    "foobar",
    "Comparison between keys should be case-insensitive"
)]
fn rkey_pair_cmp_key_success_test<K, V>(
    #[case] key: K,
    #[case] val: V,
    #[case] cmp_key: K,
    #[case] desc: &str,
) where
    K: ToString,
    V: Into<RValue> + Clone,
{
    assert!(RKeyPair::new(key, val).cmp_key(&cmp_key), "{desc}");
}

#[rstest]
#[case("foo", "foo", "bar", "Comparing 'foo' and 'bar' should be false")]
#[case(1, 1, 2, "Comparing '1' and '2' should be false")]
fn rkey_pair_cmp_key_failure_test<K, V>(
    #[case] key: K,
    #[case] val: V,
    #[case] cmp_key: K,
    #[case] desc: &str,
) where
    K: ToString,
    V: Into<RValue> + Clone,
{
    assert!(!RKeyPair::new(key, val).cmp_key(&cmp_key), "{desc}");
}

#[repr(u8)]
#[derive(StableAbi, Clone, PartialEq, Eq, Debug)]
pub enum RValKind {
    Null,
    Int,
    Float,
    Bool,
    Str,
    List,
    Obj,
    Html,
}

#[repr(C)]
#[derive(StableAbi, Clone, PartialEq, Eq, Debug)]
pub struct RState {
    pub(crate) pairs: RVec<RKeyPair>,
}

impl RState {
    pub fn new() -> RState {
        RState { pairs: RVec::new() }
    }

    /// Returns if the given key is in the map.
    /// Only checks the first-level.
    ///
    /// ```rust
    /// use foreign_std_lib::state::rs_state::RState;
    /// let key = String::from("foo");
    /// assert!(RState::new().insert(&key, 0).contains_key(&key));
    /// assert!(!RState::new().contains_key(&key));
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
    /// use foreign_std_lib::state::rs_state::RState;
    /// let key = String::from("foo");
    /// assert_eq!(RState::new().merge(RState::new()), RState::new());
    /// assert_eq!(
    ///     RState::new().merge(RState::new().insert(&key, 0)),
    ///     RState::new().insert(&key, 0)
    /// );
    /// assert_eq!(
    ///     RState::new().insert(&key, 0).merge(RState::new()),
    ///     RState::new().insert(&key, 0)
    /// );
    /// ```
    pub fn merge(self, other: Self) -> Self {
        let mut pairs = self.pairs.clone();
        let mut other_pairs = other
            .pairs
            .into_iter()
            .filter(|pk| !self.contains_key(&pk.key))
            .collect();
        pairs.append(&mut other_pairs);
        Self { pairs }
    }

    // TODO: Add doc-test
    pub fn merge_mut(&mut self, other: Self) {
        self.pairs.append(
            &mut other
                .pairs
                .into_iter()
                .filter(|pk| !self.contains_key(&pk.key))
                .collect(),
        )
    }

    /// Inserts the given value to the given key.
    /// If it already exists in the map, updates the value instead.
    ///
    /// ```rust
    /// use foreign_std_lib::state::rs_state::RState;
    /// let key = String::from("foo");
    /// let other_key = String::from("foobar");
    /// let mut a = RState::new();
    /// a.insert_mut(&key, 1);
    /// let mut b = RState::new();
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
    /// use foreign_std_lib::state::rs_state::RState;
    /// let key = String::from("foo");
    /// let other_key = String::from("foobar");
    /// assert_eq!(RState::new().insert(&key, 1), RState::new().insert(&key, 1));
    /// assert!(RState::new().insert(&key, 1).lookup(&key).is_some());
    /// assert!(RState::new().insert(&key, 1).lookup(&other_key).is_none());
    /// ```
    pub fn insert<S, T>(self, key: &S, val: T) -> Self
    where
        S: ToString + ?Sized,
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
                pairs.push(RKeyPair::new(key.to_string(), val));
                pairs
            },
        }
    }

    /// Checks the map for the given key, if it exists, returns the value. If
    /// it doesn't exist, returns none.
    ///
    /// ```rust
    /// use foreign_std_lib::state::rs_state::RState;
    /// let key = String::from("foo");
    /// let other_key = String::from("foobar");
    /// assert!(RState::new().insert(&key, 1).lookup(&key).is_some());
    /// assert!(RState::new().insert(&key, 1).lookup(&other_key).is_none());
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
    /// use foreign_std_lib::state::rs_state::RState;
    /// let key = String::from("foo");
    /// let mut map = RState::new().insert(&key, 1);
    /// assert!(map.lookup(&key).is_some());
    /// assert!(map.remove_mut(&key).is_some());
    /// assert!(RState::new().remove_mut(&key).is_none());
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
    /// use foreign_std_lib::state::rs_state::RState;
    /// let key = String::from("foo");
    /// let mut map = RState::new().insert(&key, 1);
    /// assert!(map.lookup(&key).is_some());
    /// assert!(map.remove(&key).lookup(&key).is_none());
    /// assert!(RState::new().remove(&key).lookup(&key).is_none());
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

#[cfg(test)]
static EMPTY_VEC: &[(String, RValue); 0] = &[];

#[cfg(test)]
fn rvalue_test_vec() -> Vec<(&'static str, RValue)> {
    vec![
        ("foo", "bar".into()),
        ("foobar", 1.into()),
        ("foobaz", (0..10).collect::<Vec<_>>().into()),
        ("bazfoo", vec![("foo", "bar")].into()),
    ]
}

#[rstest]
#[case(EMPTY_VEC.to_vec())]
#[case(vec![("foo", "bar".into())])]
#[case(vec![("foo", 1.into())])]
#[case(rvalue_test_vec())]
fn building_rs_state_test<K>(#[case] keyvals: Vec<(K, RValue)>)
where
    K: ToString,
{
    let mut map = RState::new();
    for (k, v) in keyvals.iter() {
        map = map.insert(k, v.clone());
        assert!(map.lookup(k).is_some());
        assert!(map.lookup(k).into_option().map(|val| val.eq(v)).unwrap());
    }
    assert!(keyvals.iter().all(|(k, _)| map.lookup(k).is_some()));
}
