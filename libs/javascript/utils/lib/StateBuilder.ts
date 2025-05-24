import type { Instruction } from "./Instruction";
import type { CoreModification } from "./CoreModification";
import { combine, flatten, isAdd, isNoOp } from "./InstructionHelper";
import type { Value } from "./Value";
import { isValue, tValueMaybe, type ValuePrimitive } from "./Types";
import * as O from "fp-ts/Option";
import type { State } from "./State";

export class StateBuilder {
  private state: Instruction<Value> = "noOp";

  add(field: string, value: Value | ValuePrimitive): StateBuilder {
    if (isValue(value)) {
      this.state = combine(this.state, { add: [field, value] });
    } else {
      this.state = combine(
        this.state,
        { add: [field, O.getOrElse((): Value => "null")(tValueMaybe(value))] }
      );
    }
    return this;
  }

  set(field: string, value: Value | ValuePrimitive): StateBuilder {
    return this.rem(field, value).add(field, value);
  }

  rem(field: string, value?: Value | ValuePrimitive): StateBuilder {
    if (value === undefined) {
      this.state = combine(this.state, { rem: [field, "null"] });
    } else if (isValue(value)) {
      this.state = combine(this.state, { rem: [field, value] });
    } else {
      this.state = combine(
        this.state,
        { rem: [field, O.getOrElse((): Value => "null")(tValueMaybe(value))] }
      );
    }
    return this;
  }

  toState(): State {
    const state: State = {};
    const xs = flatten(this.state);
    xs.filter(x => !isNoOp(x))
      .forEach(x => {
        if (isAdd(x)) {
          const [k, v] = x.add;
          state[k] = v;
        } else {
          const [k, _] = x.rem;
          state[k] = undefined;
        }
      });
    return state;
  }

  build(): CoreModification {
    return {
      state: this.state,
      ui: ["noOp", "noOp", "noOp"]
    };
  }
}
