use crate::attrs::Attr;
use crate::instruction::inst::Instruction;
use crate::state::Value;
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
    pub fn from_instr(
        state: Instruction<Value>,
        ui: (Instruction<Html>, Instruction<String>, Instruction<Attr>),
    ) -> Self {
        Self { state, ui }
    }

    pub fn ui(ui: UIInstructionBuilder) -> Self {
        CoreModification {
            state: Instruction::NoOp,
            ui: ui.instruction(),
        }
    }

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

    pub fn get_attr_instr(&self) -> Instruction<Attr> {
        self.ui.2.clone()
    }

    pub fn get_html_instr(&self) -> Instruction<Html> {
        self.ui.0.clone()
    }
}
