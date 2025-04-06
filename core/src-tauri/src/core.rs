use core_std_lib::{core::Core, html::Html, state::State};

use crate::statics::{NMIDE_STATE, NMIDE_UI};

pub struct NmideCore;

impl Core for NmideCore {
    async fn state(&self) -> State {
        let st = NMIDE_STATE.read().await;
        st.clone()
    }

    async fn ui(&self) -> Html {
        let ui = NMIDE_UI.read().await;
        ui.clone()
    }

    async fn throw_event(&self, event: core_std_lib::event::Event) {
        todo!()
    }
}
