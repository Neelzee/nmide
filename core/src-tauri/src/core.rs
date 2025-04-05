use core_std_lib::{core::Core, html::Html, state::State};

pub struct NmideCore;

impl Core for NmideCore {
    async fn state(&self) -> State {
        todo!()
    }

    async fn ui(&self) -> Html {
        todo!()
    }

    async fn throw_event(&self, event: core_std_lib::event::Event) {
        todo!()
    }
}
