use crate::html::Html;
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
#[ts(export_to = "State.ts")]
pub enum Value {
    #[default]
    Null,
    Int(i32),
    #[ts(as = "f32")]
    Float(NotNan<f32>),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    #[ts(type = "Record<string, Value | undefined>")]
    Obj(HHMap),
    Html(Html),
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
        Self(HashableHashMap::from_iter(value))
    }
}

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

    pub fn rem<S: Into<String>>(self, field: S) -> Self {
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

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Self::Obj(value.into())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, TS)]
#[ts(export, type = "Record<string, Value | undefined>")]
pub struct State(HashMap<String, Value>);

impl State {
    pub fn get<S: ToString>(&self, field: S) -> Option<Value> {
        let field = field.to_string();
        let fields = field.split(".").collect::<Vec<_>>();

        if fields.len() == 1 {
            return self.0.get(&(field.to_string())).cloned();
        }

        let mut fields = fields.into_iter();

        let mut map = self.0.clone();

        while let Some(field) = fields.next() {
            match map.clone().get(field) {
                Some(v) if v.is_obj() => {
                    map = v.clone().obj().unwrap();
                }
                Some(v) if fields.clone().count() == 0 => {
                    return Some(v.clone());
                }
                _ => {
                    return None;
                }
            }
        }
        return None;
    }

    pub fn build() -> StateInstructionBuilder {
        StateInstructionBuilder::default()
    }

    pub fn inner(self) -> HashMap<String, Value> {
        self.0
    }

    /// Adds value to field.
    ///
    /// Similar to JSON, `dot` means a new, nested object.
    ///
    /// ```rust
    /// use core_std_lib::state::{Value, State};
    /// let mut state = State::default();
    /// state = state.add("foo", Value::new_obj().obj_add("bar", Value::Int(0)));
    /// assert_eq!(state, State::default().add("foo.bar", Value::Int(0)));
    /// ```
    pub fn add<S: ToString>(self, field: S, value: Value) -> Self {
        let mut map = self.0;
        let field = field.to_string();

        let mut fields = field.split(".").collect::<Vec<_>>();

        if fields.len() == 1 {
            let new_val = match map.get(&field) {
                Some(Value::List(xs)) => {
                    let mut ys = xs.clone();
                    ys.push(value);
                    Value::List(ys)
                }
                _ => value,
            };
            map.insert(field, new_val);
            return Self(map);
        }

        let last = fields.pop().unwrap();
        let first = fields.remove(0);

        let mut obj = Value::new_obj().obj_add(last, value);

        while let Some(f) = fields.pop() {
            obj = Value::new_obj().obj_add(f, obj);
        }

        map.insert(first.to_string(), obj);
        return Self(map);
    }

    fn _rem(mut map: HashMap<String, Value>, fields: &[String]) -> Value {
        if fields.is_empty() {
            return Value::Obj(HHMap::from(map));
        } else if fields.len() == 1 {
            let field = fields[0].clone();
            map.remove(&field);
            return Value::Obj(HHMap::from(map));
        }
        let field = fields[0].clone();
        match map.get(&field) {
            Some(o) if o.is_obj() => {
                let inner_map = o.obj().unwrap();
                map.insert(field, Self::_rem(inner_map, &fields[1..]));
                Value::Obj(HHMap::from(map))
            }
            _ => Value::Obj(HHMap::from(map)),
        }
    }

    pub fn rem<S: ToString>(self, field: S) -> Self {
        let map = self.0;
        let field = field.to_string();
        let fields: Vec<String> = field.split(".").map(|s| s.to_string()).collect();
        Self(Self::_rem(map, &fields).obj().unwrap())
    }
}

#[derive(Default)]
// TODO: Allow for dot-notation on field access
pub struct StateInstructionBuilder(Instruction<Value>);

impl StateInstructionBuilder {
    pub fn combine(self, other: Self) -> Self {
        Self(self.0.combine(other.0))
    }

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

    pub fn set<S: ToString>(self, field: S, value: Value) -> Self {
        self.remove(field.to_string().clone())
            .add(field.to_string(), value)
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
