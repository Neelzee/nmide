import gleam/pair
import gleam/result
import gleam/list
import gleam/dict

pub type Instr(a) {
  NoOp
  Add(String, a)
  Rem(String, a)
  Then(Instr(a), Instr(a))
}

pub fn combine(l: Instr(a), r: Instr(a)) -> Instr(a) {
  case l, r {
    NoOp, _ -> r
    _, NoOp -> l
    _, _ -> Then(l, r)
  }
}

pub fn flatten(instr: Instr(a)) -> List(Instr(a)) {
  case instr {
    Then(fst, snd) -> list.append(flatten(fst), flatten(snd))
    _ -> [instr]
  }
}

fn is_noop(i: Instr(a)) -> Bool {
  case i {
    NoOp -> True
    _ -> False
  }
}


fn populate_map(dict: dict.Dict(String, Int), i: Instr(a)) -> dict.Dict(String, Int) {
  dict
  |> case i {
    Add(k, _) -> fn(d) {
      let val = dict.get(d, k)
        |> result.lazy_unwrap(fn() { 0 })
      dict.insert(d, k, val + 1)
    }
    Rem(k, _) -> fn(d) {
      let val = dict.get(d, k)
        |> result.lazy_unwrap(fn() { 0 })
      dict.insert(d, k, val - 1)
    }
    _ -> fn(d) { d }
  }
}

fn validate_instr(pair: #(Instr(a), dict.Dict(String, Int)), instr: Instr(a)) -> #(Instr(a), dict.Dict(String, Int)) {
  let #(acc, map) = pair
  case instr {
    Add(k, _) -> {
      let val = dict.get(map, k)
        |> result.lazy_unwrap(fn() { 1 })
      case val {
        _ if val <= 1 -> #(combine(acc, instr), map)
        _ -> pair
      }
    }
    Rem(k, _) -> {
      let val = dict.get(map, k)
        |> result.lazy_unwrap(fn() { -1 })
      case val {
        _ if val <= -1 -> #(combine(acc, instr), map)
        _ -> pair
      }
    }
    _ -> #(combine(acc, instr), map)
  }
}

pub fn opt(instrs: List(Instr(a))) -> Instr(a) {
  let xs = list.filter(instrs, is_noop)

  list.fold(xs, #(NoOp, list.fold(xs, dict.new(), populate_map)), validate_instr)
  |> pair.first
}