import { Html } from "./Html";
import { Attr } from "./Attr";
import { Instruction } from "./Instruction";
import { CoreModification } from "./CoreModification";
import { combine } from "./InstructionHelper";

export class HtmlBuilder {
  private node: Instruction<Html>;
  private text: Instruction<string>;
  private attr: Instruction<Attr>;

  constructor() {
    this.node = "noOp";
    this.text = "noOp";
    this.attr = "noOp";
  }

  add(node: Html, id: string | null, cls: string | null): HtmlBuilder {
    this.node = combine(this.node, { add: [id, cls, node] });
    return this;
  }

  build(): CoreModification {
    return {
      state: "noOp",
      ui: [this.node, this.text, this.attr]
    };
  }
}