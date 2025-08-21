import gleam/option.{type Option}
import gleam/list
import gleam/javascript/array.{type Array,from_list,map,to_list}
import gleam/pair
import gleam/result
import nmide/value
import nmide/html.{type Html, Html, type Attr}
import nmide/event.{type Event, Event}
import nmide/state.{type State, State}

pub type Object = Array(#(String, value.Value))

pub fn de_objectify(object: Object) -> value.Value {
  object
  |> to_list
  |> value.Obj
}

pub fn get(obj: Object, field: String) -> Option(value.Value) {
  obj
  |> to_list
  |> list.key_find(field)
  |> option.from_result
}

fn value_to_html(val: value.Value) -> Html {
  case val {
    value.Obj(xs) -> to_html(from_list(xs))
    _ -> Html(kind: "div", kids: [], attrs: [], text: option.None)
  }
}

fn has_key(k: String) -> fn(#(String, _)) -> Bool {
  fn(p) { pair.first(p) == k }
}

fn value_to_attr(val: value.Value) -> option.Option(Attr) {
  case val {
    value.Obj(xs) -> list.find(xs, has_key("id"))
      |> result.or(list.find(xs, has_key("class")))
      |> result.or(list.find(xs, has_key("onClick")))
      |> option.from_result
      |> fn(x) {
        case x {
          option.Some(#("class", value.Str(v))) -> option.Some(html.Class(v))
          option.Some(#("id", value.Str(v))) -> option.Some(html.Id(v))
          option.Some(#("onClick", value.Obj(o))) -> value_to_event(value.Obj(o))
            |> option.map(html.OnClick)
          _ -> panic as "This should not happen"
          }
      }
    _ -> option.None
  }
}

fn value_to_event(val: value.Value) -> option.Option(Event) {
  let o = case val {
    value.Obj(xs) -> xs
    _ -> []
  }
  let event = option.from_result(list.find(o, has_key("event_name")))
  |> option.map(pair.second)
  |> option.then(value.str)
  let module = option.from_result(list.find(o, has_key("module_name")))
  |> option.map(pair.second)
  |> option.then(value.str)
  let args = option.from_result(list.find(o, has_key("args")))
  |> option.map(pair.second)
  case event, module
  {
    option.None, _ -> option.None
    _, option.None -> option.None
    option.Some(event), option.Some(module) -> option.Some(Event(event:, module:, args:))
  }
}

fn unsafe_unwrap(option: option.Option(a)) -> a {
  case option {
    option.Some(a) -> a
    option.None -> panic as "unwrapped none value"
  }
}

/// Creates an Html instance from an Object, will always suceed.
/// default `kind`: div
/// default `kids`: []
/// default `attrs`: []
/// default `text`: None
pub fn to_html(obj: Object) -> Html {
  Html(
    kind: get(obj, "kind")
      |> option.then(value.str)
      |> option.unwrap("div"),
    kids: get(obj, "kids")
      |> option.then(value.list)
      |> option.map(fn(xs) { list.map(xs, value_to_html) })
      |> option.unwrap([]),
    attrs: get(obj, "attrs")
      |> option.then(value.list)
      |> option.unwrap([])
      |> list.map(value_to_attr)
      |> list.filter(option.is_some)
      |> list.map(unsafe_unwrap),
    text: get(obj, "text")
      |> option.then(value.str)
  )
}

pub fn state_from_obj(obj: Object) -> State {
  State(to_list(obj))
}