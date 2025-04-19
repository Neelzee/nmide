use crate::{
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{State, StateInstructionBuilder},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::attrs::Attr;
use crate::instruction::Instruction;
use crate::state::Value;
use tokio::sync::mpsc::{Sender};

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

    async fn get_sender(&self) -> Sender<CoreModification>;
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CoreModification {
    state: Instruction<Value>,
    ui: (Instruction<Html>, Instruction<String>, Instruction<Attr>),
}

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
        let (mut node, mut text, mut attr) = self.ui;
        let (n,  t,  a) = other.ui;
        Self {
            state: self.state.combine(other.state),
            ui: (node.combine(n), text.combine(t), attr.combine(a))
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
