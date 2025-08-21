import gleam/dict
import gleam/utils/value

pub type State {
  State(dict.Dict(String, value.Value))
}