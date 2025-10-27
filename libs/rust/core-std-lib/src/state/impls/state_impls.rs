use crate::state::{state_builder::StateBuilder, HHMap, State, Value};
use std::collections::HashMap;

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
        None
    }

    pub fn build() -> StateBuilder {
        StateBuilder::default()
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
        Self(map)
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

    pub fn remove<S: ToString>(self, field: S) -> Self {
        let map = self.0;
        let field = field.to_string();
        let fields: Vec<String> = field.split(".").map(|s| s.to_string()).collect();
        Self(Self::_rem(map, &fields).obj().unwrap())
    }
}
