use std::future::Future;

use crate::{event::REvent, html::rhtml::RHtml, map::rmap::RMap};

pub trait Core {
    fn get_ui() -> impl Future<Output = RHtml>;
    fn get_state() -> impl Future<Output = RMap>;
    fn throw_event(event: REvent) -> impl Future<Output = ()>;

    fn apply_modification(self, mod: CoreModification) -> Self {
        todo!("Implement modification application")
    }
}
