import * as t from "io-ts";
import { type Html, type Instruction, type Attr, type Value } from "@nmide/js-utils";
import { DAttr, DHtml } from "./html_decoder";
import { DValue } from "./value_decoder";

const DInstr = <T extends t.Mixed>(name: string, a: T): t.Type<Instruction<t.TypeOf<T>>> =>
  t.recursion(name, () =>
    t.union([
      t.literal("noOp"),
      t.type({ add: t.tuple([t.string, a]) }),
      t.type({ rem: t.tuple([t.string, a]) }),
      t.type({ then: t.tuple([DInstr(name, a), DInstr(name, a)]) }),
    ])
  );

export const DInstrHtml: t.Type<Instruction<Html>> = DInstr("DInstrHtml", DHtml);
export const DInstrString: t.Type<Instruction<string>> = DInstr("DInstrHtml", t.string);
export const DInstrAttr: t.Type<Instruction<Attr>> = DInstr("DInstrAttr", DAttr);
export const DInstrValue: t.Type<Instruction<Value>> = DInstr("DInstrValue", DValue);