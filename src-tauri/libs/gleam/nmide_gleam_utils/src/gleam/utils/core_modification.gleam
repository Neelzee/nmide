import gleam/utils/value
import gleam/utils/instr

pub type CoreModification {
  CoreModification(
    instr.Instr(value.Value),
    instr.Instr(value.Html),
    instr.Instr(value.Attr),
    instr.Instr(String),
  )
}