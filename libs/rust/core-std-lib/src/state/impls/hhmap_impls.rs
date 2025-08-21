use crate::state::{HHMap, Value};
use hashable::HashableHashMap;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;

impl PartialOrd for HHMap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HHMap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mps = self.clone().to_hm();
        let mpo = other.clone().to_hm();
        mps.iter().cmp(mpo.iter())
    }
}

impl Serialize for HHMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.0.clone().into_iter() {
            map.serialize_entry(&k, &v)?;
        }
        map.end()
    }
}

struct HHMapVisitor;

impl<'de> Visitor<'de> for HHMapVisitor {
    type Value = HHMap;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("A standard object")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = HHMap(HashableHashMap::new());

        while let Some((key, value)) = access.next_entry()? {
            map.0.insert(key, value);
        }

        Ok(map)
    }
}

impl<'de> Deserialize<'de> for HHMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(HHMapVisitor)
    }
}

impl HHMap {
    pub fn to_hm(self) -> HashMap<String, Value> {
        HashMap::from_iter(self.0.into_iter().map(|(k, v)| (k.clone(), v.clone())))
    }
}

impl From<HashMap<String, Value>> for HHMap {
    fn from(value: HashMap<String, Value>) -> Self {
        Self(HashableHashMap::from_iter(value))
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Self::Obj(value.into())
    }
}
