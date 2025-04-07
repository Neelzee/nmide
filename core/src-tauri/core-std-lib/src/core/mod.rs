use crate::{
    event::Event,
    html::{Html, UIInstruction, UIInstructionBuilder},
    state::{State, StateInstruction, StateInstructionBuilder},
};
use async_trait::async_trait;

pub(crate) mod modification;

#[async_trait]
pub trait Core: Send + Sync {
    async fn state(&self) -> State;
    async fn ui(&self) -> Html;
    async fn throw_event(&self, event: Event);
}

#[derive(Default)]
pub struct CoreModification {
    state_inst: StateInstruction,
    ui_inst: UIInstruction,
}

impl CoreModification {
    pub fn set_state(self, builder: StateInstructionBuilder) -> Self {
        Self {
            state_inst: builder.instruction(),
            ..self
        }
    }

    pub fn set_ui(self, builder: UIInstructionBuilder) -> Self {
        Self {
            ui_inst: builder.instruction(),
            ..self
        }
    }

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

    pub fn build_state(self, state: State) -> (State, UIInstructionBuilder) {
        (
            StateInstructionBuilder::new(self.state_inst).build(state),
            UIInstructionBuilder::new(self.ui_inst),
        )
    }
}
