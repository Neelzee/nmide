use std::str::FromStr;

use crate::state::rs_state::RValue;
use abi_stable::{
    StableAbi,
    std_types::{ROption, RString},
};
use core_std_lib::event::Event;

#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct REvent {
    event_name: RString,
    args: ROption<RValue>,
}

impl From<Event> for REvent {
    fn from(value: Event) -> Self {
        Self {
            event_name: RString::from_str(value.event_name()).unwrap_or_default(),
            args: if let Some(a) = value.args() {
                ROption::RSome(a.clone().into())
            } else {
                ROption::RNone
            },
        }
    }
}

impl REvent {
    pub fn event_name(&self) -> &RString {
        &self.event_name
    }

    pub fn to_event(&self) -> Event {
        Event::new(
            self.event_name.as_str(),
            if let ROption::RSome(arg) = self.args.clone() {
                Some(arg.to_value())
            } else {
                None
            },
        )
    }
}
