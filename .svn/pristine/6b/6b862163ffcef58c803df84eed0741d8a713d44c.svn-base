import nmide/event.{type Event}
import nmide/state.{type State}
import nmide/html.{type Html}
import gleam/javascript/promise
import nmide/modules
import gleam/option

pub type Core {
  Core(
    state: State,
    ui: Html,
    throw_event: fn(Event) -> promise.Promise(Nil),
    registrate_handler: fn(
      modules.Module,
      option.Option(String),
      option.Option(String)
    ) -> promise.Promise(Nil),
  )
}
