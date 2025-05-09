import nmide/event.{type Event}
import gleam/option.{type Option}
import nmide/instruction.{type Instruction}

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

pub type UIInstruction =
  #(Instruction(Html), Instruction(Attr), Instruction(String))
