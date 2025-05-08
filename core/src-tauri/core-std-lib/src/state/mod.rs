use crate::instruction::inst::Instruction;
use hashable::HashableHashMap;
use ordered_float::NotNan;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;
use ts_rs::TS;

pub mod impls;

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Ord, Serialize, Deserialize, TS, Hash, Eq,
)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum Value {
    #[default]
    Null,
    Int(i32),
    #[ts(as = "f32")]
    Float(NotNan<f32>),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    #[ts(type = "{ [key in string]?: Value }")]
    Obj(HHMap),
}

#[derive(Default, Debug, Clone, PartialEq, Hash, Eq)]
pub struct HHMap(HashableHashMap<String, Value>);

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
        Self(HashableHashMap::from_iter(value.into_iter()))
    }
}

impl Value {
    pub fn bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
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

    pub fn int(self) -> Option<i32> {
        match self {
            Self::Int(i) => Some(i),
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

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Self::Obj(value.into())
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct State(HashMap<String, Value>);

impl State {
    pub fn get<S: ToString>(&self, field: S) -> Option<&Value> {
        self.0.get(&(field.to_string()))
    }

    pub fn build() -> StateInstructionBuilder {
        StateInstructionBuilder::default()
    }

    pub fn inner(self) -> HashMap<String, Value> {
        self.0
    }

    pub(crate) fn add<S: ToString>(self, field: S, value: Value) -> Self {
        let mut map = self.0;
        let field = field.to_string();

        map.insert(field, value);
        return Self(map);
    }

    pub(crate) fn rem<S: ToString>(self, field: S) -> Self {
        let mut map = self.0;
        map.remove(&field.to_string());
        Self(map)
    }
}

#[derive(Default)]
// TODO: Allow for dot-notation on field access
pub struct StateInstructionBuilder(Instruction<Value>);

impl StateInstructionBuilder {
    pub fn instruction(&self) -> Instruction<Value> {
        self.0.clone()
    }

    pub(crate) fn new(instruction: Instruction<Value>) -> Self {
        Self(instruction)
    }

    pub fn add<S: ToString>(self, field: S, value: Value) -> Self {
        Self(self.0.combine(Instruction::Add(field.to_string(), value)))
    }

    pub fn remove<S: ToString>(self, field: S) -> Self {
        Self(
            self.0
                .combine(Instruction::Rem(field.to_string(), Value::default())),
        )
    }

    pub fn set(self, field: String, value: Value) -> Self {
        self.remove(field.clone()).add(field, value)
    }

    // HACK: `Panic`king is done instead of having a type-level error handling, to make it
    // easier to implement
    // TODO: Make type-level error handling
    pub fn build(self, state: State) -> State {
        Self::eval(self.0, state)
    }

    fn eval(instruction: Instruction<Value>, state: State) -> State {
        match instruction {
            Instruction::Add(field, value) => state.add(field, value),
            Instruction::Rem(field, _) => state.rem(field),
            Instruction::Then(f, s) => Self::eval(*s, Self::eval(*f, state)),
            _ => state,
        }
    }
}
