import type { CoreModification } from "@nmide/js-utils";
import * as t from "io-ts";
import { DInstrAttr, DInstrHtml, DInstrString, DInstrValue } from "./instr_decoder";

export const DCoreModification: t.Type<CoreModification> = t.type({
  state: DInstrValue,
  ui: t.tuple([DInstrHtml, DInstrString, DInstrAttr])
})