use crate::{event::Event, state::Value};

impl Event {
    pub fn post_init() -> Self {
        Self::PostInit
    }

    pub fn pre_exit() -> Self {
        Self::PreExit
    }

    pub fn core_response<S: Into<String>>(event: S, args: Option<Value>) -> Self {
        Self::CoreResponse {
            event: event.into(),
            args,
        }
    }
}
