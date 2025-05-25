import type { Instruction } from "./Instruction";
import type { CoreModification } from "./CoreModification";
import { combine, flatten, isAdd, isNoOp } from "./InstructionHelper";
import type { Value } from "./Value";
import type { State } from "./State";
import type { UiBuilder } from "./UiBuilder";

export class StateBuilder {
  private state: Instruction<Value> = "noOp";

  add(field: string, value: Value): StateBuilder {
    this.state = combine(this.state, { add: [field, value] });
    return this;
  }

  set(field: string, value: Value): StateBuilder {
    return this.rem(field).add(field, value);
  }

  rem(field: string, value?: Value): StateBuilder {
    this.state = combine(this.state, { rem: [field, value || "null"] });
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

  build(ui?: UiBuilder): CoreModification {
    return {
      state: this.state,
      ui: ui?.build()?.ui || ["noOp", "noOp", "noOp"]
    };
  }
}
