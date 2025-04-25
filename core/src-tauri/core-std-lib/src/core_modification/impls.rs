use crate::instruction::inst::Instruction;
use crate::{
    core_modification::CoreModification,
    html::{Html, UIInstructionBuilder},
    state::{State, StateInstructionBuilder},
};

impl Default for CoreModification {
    fn default() -> Self {
        Self {
            state: Instruction::NoOp,
            ui: (Instruction::NoOp, Instruction::NoOp, Instruction::NoOp),
        }
    }
}

impl CoreModification {
    pub fn append(a: Self, b: Self) -> Self {
        a.combine(b)
    }

    pub fn set_state(self, builder: StateInstructionBuilder) -> Self {
        Self {
            state: builder.instruction(),
            ..self
        }
    }

    pub fn set_ui(self, builder: UIInstructionBuilder) -> Self {
        Self {
            ui: builder.instruction(),
            ..self
        }
    }

    pub fn combine(self, other: Self) -> Self {
        let (node, text, attr) = self.ui;
        let (n, t, a) = other.ui;
        Self {
            state: self.state.combine(other.state),
            ui: (node.combine(n), text.combine(t), attr.combine(a)),
        }
    }

    pub fn build(self, state: State, ui: Html) -> (State, Html) {
        (
            StateInstructionBuilder::new(self.state).build(state),
            UIInstructionBuilder::new(self.ui).build(ui),
        )
    }

    pub fn build_state(self, state: State) -> (State, UIInstructionBuilder) {
        (
            StateInstructionBuilder::new(self.state).build(state),
            UIInstructionBuilder::new(self.ui),
        )
    }
}
