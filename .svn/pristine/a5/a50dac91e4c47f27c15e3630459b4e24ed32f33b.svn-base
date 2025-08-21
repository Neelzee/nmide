import type { Html } from "./Html";
import type { Attr } from "./Attr";
import type { Instruction } from "./Instruction";
import type { CoreModification } from "./CoreModification";
import { combine } from "./InstructionHelper";
import { HtmlBuilder } from "./HtmlBuilder";
import type { StateBuilder } from "./StateBuilder";

export class UiBuilder {
  private node: Instruction<Html>;
  private text: Instruction<string>;
  private attr: Instruction<Attr>;

  constructor() {
    this.node = "noOp";
    this.text = "noOp";
    this.attr = "noOp";
  }

  add(node: Html | HtmlBuilder, id?: string): UiBuilder {
    this.node = combine(
      this.node,
      {
        add: [
          id === undefined ? "" : id,
          node instanceof HtmlBuilder ? node.build() : node
        ]
      }
    );
    return this;
  }

  add_attr(attr: Attr, id?: string): UiBuilder {
    this.attr = combine(
      this.attr,
      {
        add: [
          id === undefined ? "" : id,
          attr,
        ]
      }
    );
    return this;
  }

  rem_attr(attr: Attr, id?: string): UiBuilder {
    this.attr = combine(
      this.attr,
      {
        rem: [
          id === undefined ? "" : id,
          attr,
        ]
      }
    );
    return this;
  }

  build(builder?: StateBuilder): CoreModification {
    const supplied_state = builder?.build().state;
    const state = supplied_state === undefined ? "noOp" : supplied_state;
    return {
      state,
      ui: [this.node, this.text, this.attr]
    };
  }
}