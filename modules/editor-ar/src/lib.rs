use std::collections::HashMap;

use core_std_lib::state::{HHMap, Value};

pub enum Art {
    Token {
        pos: Position,
        char: char,
    },
    Group {
        pos: Position,
        group: Vec<Art>,
        metadata: HashMap<String, Value>,
    },
}

pub struct Position {
    start_pos: usize,
    end_pos: usize,
}

impl Position {
    pub fn from_value(value: Value) -> Option<Self> {
        match value {
            Value::Obj(mp) => {
                let map = mp.to_hm();
                let start_pos = map.get("start_pos").cloned().and_then(|a| a.int())?;
                let end_pos = map.get("end_post").cloned().and_then(|a| a.int())?;

                let start_pos = start_pos.unsigned_abs() as usize;
                let end_pos = end_pos.unsigned_abs() as usize;

                Some(Self { start_pos, end_pos })
            }
            _ => None,
        }
    }

    pub fn to_value(self) -> Value {
        Value::new_obj()
            .add("start_pos", Value::Int(self.start_pos as i32))
            .add("end_pos", Value::Int(self.end_pos as i32))
    }
}

impl Art {
    pub fn parse(s: &str) -> Self {
        Art::Group {
            pos: Position {
                start_pos: 0,
                end_pos: s.len(),
            },
            group: s
                .chars()
                .enumerate()
                .map(|(i, c)| Art::Token {
                    pos: Position {
                        start_pos: i,
                        end_pos: i + 1,
                    },
                    char: c,
                })
                .collect::<Vec<_>>(),
            metadata: HashMap::new(),
        }
    }

    pub fn from_value(value: Value) -> Option<Self> {
        match value {
            Value::Obj(mp) => {
                let map = mp.to_hm();
                let pos = Position::from_value(map.get("pos").cloned()?)?;

                if let Some(s) = map.get("char").and_then(|v| v.str()) {
                    return Some(Self::Token {
                        pos,
                        char: s.chars().find(|_| true)?,
                    });
                }

                if let Some(s) = map.get("group").and_then(|v| v.list()) {
                    return Some(Self::Group {
                        pos,
                        group: s
                            .into_iter()
                            .map(Self::from_value)
                            .collect::<Option<Vec<_>>>()?,
                        metadata: map.get("metadata").and_then(|v| v.obj())?,
                    });
                }

                None
            }
            _ => None,
        }
    }

    pub fn to_value(self) -> Value {
        match self {
            Art::Token { pos, char } => Value::new_obj()
                .add("pos", pos.to_value())
                .add("char", Value::Str(char.to_string())),
            Art::Group {
                pos,
                group,
                metadata,
            } => Value::new_obj().add("pos", pos.to_value()).add(
                "group",
                Value::List(group.into_iter().map(Self::to_value).collect())
                    .add("metadata", Value::Obj(HHMap::from(metadata))),
            ),
        }
    }
}
