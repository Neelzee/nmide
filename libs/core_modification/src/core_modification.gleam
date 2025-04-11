//// Module Documentation

import gleam/javascript/promise
import gleam/list
import gleam/option.{type Option}

pub type Event {
  Event(
    event_name: String,
    module_name: String,
    args: Option(Value)
  )
}

pub type Attr {
  Id(String)
  Class(String)
  OnClick(Event)
}

pub type Html {
  Html(
    kind: String,
    kids: List(Html),
    attrs: List(Attr),
    text: Option(String)
  )
}

pub type Value {
  Int(Int)
  Float(Float)
  Bool(Bool)
  Str(String)
  List(List(Value))
  Obj(List(#(String, Value)))
}

pub type State {
  State(List(#(String, Value)))
}

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

pub type UIInstruction = #(Instruction(Html), Instruction(Attr), Instruction(String))
pub type StateInstruction = Instruction(Value)

pub fn opt_state_instructions(instructions: StateInstruction) -> StateInstruction {
  let removed_fields = rem_nodes(instructions)
  noop_opt(opt(instructions, fn(n) { modifies(n, removed_fields) }))
}

pub fn opt_ui_instructions(instructions: UIInstruction) -> UIInstruction {
  let #(html, attr, str) = instructions
  let removed_nodes = rem_nodes(html)
  let opt_attr = opt(attr, fn(n) { modifies(n, removed_nodes) })
  let opt_str = opt(str, fn(n) { modifies(n, removed_nodes) })
  #(noop_opt(html), noop_opt(opt_attr), noop_opt(opt_str))
}

fn noop_opt(inst: Instruction(a)) -> Instruction(a) {
  case inst {
    Then(f, s) -> case noop_opt(f), noop_opt(s) {
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
    Rem(id, _) |
    Mod(id, _) |
    Add(id, _) -> list.contains(nodes, id)
    _ -> False
  }
}

fn opt(inst: Instruction(a), p: fn(Instruction(a)) -> Bool) -> Instruction(a) {
  let not_pred = !p(inst)
  case inst {
    Then(f, s) -> case opt(f, p), opt(s, p) {
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

pub type CoreModification {
  CoreModification(
    state: StateInstruction,
    ui: UIInstruction,
  )
}

pub fn empty_core_modification() -> CoreModification {
  CoreModification(state: NoOp, ui: #(NoOp, NoOp, NoOp))
}

pub type Core {
  Core
}

pub type Module {
  Module(
    name: String,
    init: fn(Core) -> promise.Promise(CoreModification),
    handler: fn(Event, Core) -> promise.Promise(CoreModification),
  )
}

pub fn build_module(
  name: String,
  init: fn(Core) -> promise.Promise(CoreModification),
  handler: fn(Event, Core) -> promise.Promise(CoreModification)
) -> Module {
  Module(name, init, handler)
}