import nmide/instruction.{NoOp}
import nmide/state.{type StateInstruction}
import nmide/html.{type UIInstruction}

pub type CoreModification {
  CoreModification(state: StateInstruction, ui: UIInstruction)
}

pub fn empty_core_modification() -> CoreModification {
  CoreModification(state: NoOp, ui: #(NoOp, NoOp, NoOp))
}