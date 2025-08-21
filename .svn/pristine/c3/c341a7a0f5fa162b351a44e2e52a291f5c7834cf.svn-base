import nmide/event.{type Event}
import nmide/core.{type Core}
import nmide/core_modification.{type CoreModification}
import gleam/javascript/promise.{type Promise}

pub type Module {
  Module(
    name: String,
    init: fn(Core) -> Promise(CoreModification),
    handler: fn(Event, Core) -> Promise(CoreModification),
  )
}