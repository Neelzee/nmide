import gleam/utils/core_modification
import gleam/javascript/promise
import gleam/utils/value
import gleam/utils/state

pub type Core {
  Core(
    state: promise.Promise(state.State),
    ui: promise.Promise(value.Html),
    throw_event: fn(value.Event) -> promise.Promise(Nil),
    /// Module Name, Event Name
    registrate_handler: fn(
      String,
      String,
    ) -> promise.Promise(Nil),
    send_modification: fn(core_modification.CoreModification) -> promise.Promise(Nil)
  )
}
