import gleam/option.{type Option, Some, None}

pub type Value {
  Null
  Int(Int)
  Float(Float)
  Bool(Bool)
  Str(String)
  List(List(Value))
  Obj(List(#(String, Value)))
}

pub fn str(value: Value) -> Option(String) {
  case value {
    Str(s) -> Some(s)
    _ -> None
  }
}

pub fn list(value: Value) -> Option(List(Value)) {
  case value {
    List(x) -> Some(x)
    _ -> None
  }
}