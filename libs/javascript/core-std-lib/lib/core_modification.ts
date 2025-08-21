import { isTList, type Instruction, type State, type Value } from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";

/**
 * Parses a set of instruction, along with a starting state, into a new state.
 */
export const parseStateInstr = (instr: Instruction<Value>) => (state: State): State => {
  if ("noOp" === instr) {
    return state;
  }
  if ("add" in instr) {
    const [k, v] = instr.add;
    const old_value = state[k];
    if (isTList(old_value)) {
      const new_lst = old_value.list;
      new_lst.push(v);
      state[k] = { list: new_lst };
      return state;
    }
    state[k] = v;
    return state;
  }
  if ("rem" in instr) {
    const [k, _] = instr.rem;
    state[k] = undefined;
    return state;
  }
  const [fst, snd] = instr.then;
  return pipe(
    state,
    parseStateInstr(fst),
    parseStateInstr(snd),
  );
}
