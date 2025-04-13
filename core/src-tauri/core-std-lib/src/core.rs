use crate::{
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{State, StateInstructionBuilder},
};
use async_trait::async_trait;
use crate::attrs::Attr;
use crate::instruction::Instruction;
use crate::state::Value;

#[async_trait]
pub trait Core: Send + Sync {
    async fn state(&self) -> State;
    async fn ui(&self) -> Html;
    async fn throw_event(&self, event: Event);
    async fn add_handler(
        &self,
        event_name: Option<String>,
        module_name: Option<String>,
        handler_name: String,
    );
}

#[derive(Default)]
pub struct CoreModification {
    state_inst: Instruction<Value>,
    ui_inst: (Vec<(usize, Instruction<Html>)>, Vec<(usize, Instruction<String>)>, Vec<(usize, Instruction<Attr>)>),
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
        let (mut node, mut text, mut attr) = self.ui_inst;
        let (mut n, mut t, mut a) = other.ui_inst;
        node.append(&mut n);
        text.append(&mut t);
        attr.append(&mut a);
        Self {
            state_inst: self.state_inst.combine(other.state_inst),
            ui_inst: (node, text, attr)
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
