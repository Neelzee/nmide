import type { Instruction } from "./Instruction";
import type { CoreModification } from "./CoreModification";
import { combine } from "./InstructionHelper";
import type { Value } from "./Value";
import { isValue, tValueMaybe, type ValuePrimitive } from "./Types";
import * as O from "fp-ts/Option";

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

  build(): CoreModification {
    return {
      state: this.state,
      ui: ["noOp", "noOp", "noOp"]
    };
  }
}
