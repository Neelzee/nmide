use crate::{
    map::value::Value,
    utils::{drop_first, grab_first, lookup, remove},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod value;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, PartialOrd, Ord, TS)]
#[ts(export, export_to = "../../../app/bindings/Map.ts")]
pub struct Map(Vec<(String, Value)>);

impl Map {
    pub fn new() -> Self {
        return Map(Vec::new());
    }

    pub fn contains_key<S>(&self, key: S) -> bool
    where
        S: ToString,
    {
        self.0.iter().any(|(k, _)| k.to_string() == key.to_string())
    }

    pub fn overlap(&self, other: &Self) -> bool {
        self.0.iter().any(|(k, _)| other.contains_key(k))
    }

    // Duplicate fields will be ignored
    pub fn merge(self, other: Self) -> Self {
        let mut map = self.0;
        for (k, v) in other.0 {
            if let None = lookup(map.as_slice(), k.clone()) {
                map.push((k, v));
            }
        }
        Self(map)
    }

    pub fn lookup<S>(&self, s: S) -> Option<Value>
    where
        S: ToString,
    {
        lookup::<String, Value>(self.0.as_slice(), s.to_string()).cloned()
    }

    pub fn insert<K, V>(self, k: K, v: V) -> Self
    where
        K: ToString,
        V: Into<Value>,
    {
        let key = k.to_string();
        let mut map = self.0;
        if let Some(_) = lookup(map.as_slice(), key.clone()) {
            map = remove(map.as_slice(), key.clone());
        }
        map.push((key, v.into()));
        Self(map)
    }

    pub fn insert_lst<K, V>(self, k: K, v: V) -> Self
    where
        K: ToString,
        V: Into<Value>,
    {
        let mut map = self.0;
        match grab_first(map.as_slice(), |(key, _)| key.to_string() == k.to_string()) {
            Some((_, val)) => match val {
                Value::List(l) => {
                    let mut lst = l.clone();
                    map = drop_first(map.as_slice(), |(key, _)| key.to_string() == k.to_string());
                    lst.push(v.into());
                    map.push((k.to_string(), Value::List(lst)));
                }
                _ => (),
            },
            None => (),
        }
        Self(map)
    }
}

impl<S, T> From<Vec<(S, T)>> for Map
where
    S: ToString,
    T: Into<Value>,
{
    fn from(value: Vec<(S, T)>) -> Self {
        Map(value
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.into()))
            .collect::<Vec<(String, Value)>>())
    }
}
