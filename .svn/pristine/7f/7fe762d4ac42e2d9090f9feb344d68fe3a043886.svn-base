import gleam/list
import gleam/result.{unwrap}
import nmide/core.{type Core}
import nmide/instruction.{combine}
import nmide/core_modification.{
type CoreModification,
empty_core_modification,
CoreModification}
import gleam/javascript/array.{type Array}
import gleam/javascript/promise.{type Promise}
import nmide/modules.{type Module}
import nmide/event.{type Event}
import nmide/optimize.{opt_cm}

@external(javascript, "./externals/window.js", "get_modules")
fn get_modules() -> Array(Module)

pub fn combine_modifications(mods: Array(CoreModification)) -> CoreModification {
  coalece_modifications(array.to_list(mods))
}

fn coalece_modifications(mods: List(CoreModification)) -> CoreModification {
  list.reduce(mods, combine_cm)
  |> unwrap(empty_core_modification())
  |> opt_cm()
}

fn combine_cm(a: CoreModification, b: CoreModification) {
  CoreModification(state: combine(a.state, b.state), ui: #(
  combine(a.ui.0, b.ui.0),
  combine(a.ui.1, b.ui.1),
  combine(a.ui.2, b.ui.2),
  ))
}

fn init_module(
module: Module,
core: Core) -> promise.Promise(CoreModification) {
  module.init(core)
}

/// Initializes the given modules
pub fn init(
core: Core,
) -> promise.Promise(CoreModification) {
  get_modules()
  |> array.map(fn(m) { init_module(m, core) })
  |> promise.await_array()
  |> promise.map(fn(arr) { coalece_modifications(array.to_list(arr)) })
}

fn handle_module(
module: Module,
event: Event,
core: Core,
) -> Promise(CoreModification) {
  module.handler(event, core)
}

/// Applies the event to the handlers of the given modules
pub fn event(
event: Event,
core: Core,
) -> Promise(CoreModification) {
  get_modules()
  |> array.map(fn(m) { handle_module(m, event, core) })
  |> promise.await_array()
  |> promise.map(fn(arr) { coalece_modifications(array.to_list(arr)) })
}
