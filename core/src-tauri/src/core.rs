use core_std_lib::{core::Core, event::REvent, html::rhtml::RHtml, map::rmap::RMap};
use std::future;
use tokio::sync::RwLock;

use crate::statics::{NMIDE_STATE, NMIDE_UI};

pub struct NmideCore;

impl Core for NmideCore {
    async fn get_ui() -> RHtml {
        todo!()
    }

    async fn get_state() -> RMap {
        todo!()
    }

    /// Sends the event to another thread, to be handled and eventually modify the State or UI
    async fn throw_event(event: REvent) {
        todo!()
    }
}
