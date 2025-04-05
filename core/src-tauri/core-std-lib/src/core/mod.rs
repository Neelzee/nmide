use crate::state::StateInstruction;

pub(crate) mod modification;

pub struct Core;

pub struct CoreModification {
    state_inst: StateInstruction,
}
