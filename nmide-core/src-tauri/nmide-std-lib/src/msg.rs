use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Msg {
    PluginMsg(String, String),
}

impl Msg {
    pub fn from_string<S>(s: S) -> Msg
    where
        S: ToString,
    {
        match s.to_string().as_str() {
            "" => Self::PluginMsg(String::new(), String::new()),
            s => {
                let mut parts = s.splitn(2, char::is_whitespace);
                match (parts.next(), parts.next()) {
                    (Some(a), None) => Self::PluginMsg(s.to_string(), String::new()),
                    (Some(l), Some(r)) => Self::PluginMsg(l.to_string(), r.to_string()),
                    _ => unreachable!("Empty string case should be covered already"),
                }
            }
        }
    }
}
