use crate::{event::Event, state::Value};

impl Event {
    pub fn get(&self) -> Option<Value> {
        match self.args() {
            Some(Value::Obj(obj)) if obj.clone().to_hm().contains_key("eventArgs") => {
                Some(Value::Obj(obj.clone()))
            }
            Some(v) => Some(v.clone()),
            _ => todo!(),
        }
    }
}
