/// An Instruction on how to modify some value `value`.
pub type Instruction(value) {
  /// No Operation, results in no change to the state
  NoOp
  /// Adds the given value where the id is found.
  Add(String, value)
  /// Removes the given value where the id is found.
  Rem(String, value)
  /// Modifies the given value where the id is found.
  Mod(String, value)
  /// Combines two instruction into one
  Then(Instruction(value), Instruction(value))
}

pub fn combine(a: Instruction(t), b: Instruction(t)) -> Instruction(t) {
  case a, b {
    NoOp, b -> b
    a, NoOp -> a
    _, _ -> Then(a, b)
  }
}
