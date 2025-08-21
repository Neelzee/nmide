import { Html } from "./Html";
import { Attr } from "./Attr";
import { Instruction } from "./Instruction";
import { CoreModification } from "./CoreModification";
import { combine } from "./InstructionHelper";
import { HtmlBuilder } from "./HtmlBuilder";

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

  build(): CoreModification {
    return {
      state: "noOp",
      ui: [this.node, this.text, this.attr]
    };
  }
}