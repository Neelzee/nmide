import gleam/dict
import gleam/option

pub type Event {
  PostInit
  PreExit
  Event(
    event_name: String,
    args: option.Option(Value)
  )
}

pub type Attr {
  Id(String)
  Class(String)
  Click(Event)
}

pub type Html {
  Div(List(Html), List(Attr), option.Option(String))
  P(List(Html), List(Attr), option.Option(String))
}

pub type Value {
  Null
  Int(Int)
  Float(Float)
  Str(String)
  Bool(Bool)
  List(List(Value))
  Obj(dict.Dict(String, Value))
  Html(Html)
}