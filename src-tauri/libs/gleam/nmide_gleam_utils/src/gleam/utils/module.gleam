import gleam/utils/value
import gleam/javascript/promise
import gleam/utils/core

pub type Module {
  Module(
    name: String,
    init: fn(core.Core) -> promise.Promise(Nil),
    handler: fn(value.Event, core.Core) -> promise.Promise(Nil),
  )
}