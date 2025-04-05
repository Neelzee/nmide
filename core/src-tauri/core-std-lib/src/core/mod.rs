use crate::{
    event::Event,
    html::{Html, UIInstruction, UIInstructionBuilder},
    state::{State, StateInstruction, StateInstructionBuilder},
};

pub(crate) mod modification;

pub trait Core {
    fn state(&self) -> impl std::future::Future<Output = State>;
    fn ui(&self) -> impl std::future::Future<Output = Html>;
    fn throw_event(&self, event: Event) -> impl std::future::Future<Output = ()>;
}

#[derive(Default)]
pub struct CoreModification {
    state_inst: StateInstruction,
    ui_inst: UIInstruction,
}

impl CoreModification {
    pub fn combine(self, other: Self) -> Self {
        Self {
            state_inst: self.state_inst.combine(other.state_inst),
            ui_inst: self.ui_inst.combine(other.ui_inst),
        }
    }

    pub fn build(self, state: State, ui: Html) -> (State, Html) {
        (
            StateInstructionBuilder::new(self.state_inst).build(state),
            UIInstructionBuilder::new(self.ui_inst).build(ui),
        )
    }
}
