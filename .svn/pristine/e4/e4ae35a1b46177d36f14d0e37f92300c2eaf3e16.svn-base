import * as t from "io-ts";
import { type ModuleUnknown } from "@nmide/js-utils";

export const DModule: t.Type<ModuleUnknown> = t.type({
  name: t.string,
  init: t.Function,
  handler: t.Function,
})