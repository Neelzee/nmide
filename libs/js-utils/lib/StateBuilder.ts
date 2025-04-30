import { Instruction } from "./Instruction";
import { CoreModification } from "./CoreModification";
import { combine } from "./InstructionHelper";
import { Value } from "./Value";
import { isValue, tValueMaybe, ValuePrimitive } from "./Types";
import * as O from "fp-ts/Option";

export class StateBuilder {
  private state: Instruction<Value> = "noOp";

  add(field: string, value: Value | ValuePrimitive): StateBuilder {
    if (isValue(value)) {
      this.state = combine(this.state, { add: [field, value] });
    } else {
      this.state = combine(
        this.state,
        { add: [field, O.getOrElse((): Value => "null")(tValueMaybe(value)) ] }
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
