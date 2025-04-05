use abi_stable::sabi_trait;

use crate::{event::rs_event::REvent, html::rs_html::RHtml, state::rmap::RMap};

#[sabi_trait]
pub trait Core {
    async extern "C" fn state(&self) -> RMap;
    async extern "C" fn ui(&self) -> RHtml;
    async extern "C" fn throw_event(&self, event: REvent);
}
