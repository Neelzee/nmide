import gleam/list
import nmide/state.{type StateInstruction}
import nmide/html.{type UIInstruction}
import nmide/instruction.{type Instruction, NoOp, Add, Mod, Rem, Then}
import nmide/core_modification.{type CoreModification, CoreModification}

fn opt_state_instructions(instructions: StateInstruction) -> StateInstruction {
  let removed_fields = rem_nodes(instructions)
  noop_opt(opt(instructions, fn(n) { modifies(n, removed_fields) }))
}

fn opt_ui_instructions(instructions: UIInstruction) -> UIInstruction {
  let #(html, attr, str) = instructions
  let removed_nodes = rem_nodes(html)
  let opt_attr = opt(attr, fn(n) { modifies(n, removed_nodes) })
  let opt_str = opt(str, fn(n) { modifies(n, removed_nodes) })
  #(noop_opt(html), noop_opt(opt_attr), noop_opt(opt_str))
}

fn noop_opt(inst: Instruction(a)) -> Instruction(a) {
  case inst {
    Then(f, s) ->
    case noop_opt(f), noop_opt(s) {
      NoOp, NoOp -> NoOp
      NoOp, snd -> snd
      fst, NoOp -> fst
      fst, snd -> Then(fst, snd)
    }
    _ -> inst
  }
}

fn modifies(inst: Instruction(a), nodes: List(String)) -> Bool {
  case inst {
    Then(f, s) -> modifies(f, nodes) || modifies(s, nodes)
    Rem(id, _) | Mod(id, _) | Add(id, _) -> list.contains(nodes, id)
    _ -> False
  }
}

fn opt(inst: Instruction(a), p: fn(Instruction(a)) -> Bool) -> Instruction(a) {
  let not_pred = !p(inst)
  case inst {
    Then(f, s) ->
    case opt(f, p), opt(s, p) {
      NoOp, s -> s
      f, NoOp -> f
      f, s -> Then(f, s)
    }
    Rem(_, _) | Mod(_, _) | Add(_, _) if not_pred -> inst
    _ -> NoOp
  }
}

fn rem_nodes(inst: Instruction(a)) -> List(String) {
  case inst {
    Then(f, s) -> list.append(rem_nodes(f), rem_nodes(s))
    Rem(i, _) -> [i]
    _ -> []
  }
}

pub fn opt_cm(cm: CoreModification) -> CoreModification {
  CoreModification(
  state: opt_state_instructions(cm.state),
  ui: opt_ui_instructions(cm.ui),
  )
}
