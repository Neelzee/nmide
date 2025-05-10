import * as t from "io-ts";
import { OptionalString } from "./html_decoder";
import type { Event, Value } from "@nmide/js-utils";
import { DValueBool, DValueFloat, DValueHtml, DValueInt, DValueNull, DValueStr } from "./value_decoder";

const DDialogEvtKind = t.union([t.literal("info"), t.literal("warning"), t.literal("error")]);
const DDialogFileKind = t.union([t.literal("singleFile"), t.literal("singleDir"), t.literal("multiFile"), t.literal("saveFile"), t.literal("multiDir")]);
const DDialogBtn = t.union([t.literal("ok"), t.literal("okCancel"), t.literal("yesNo"), t.type({ okCustom: t.string }), t.type({ okCancelCustom: t.tuple([t.string, t.string]) })]);

export const DEvent: t.Type<Event> = t.recursion("DEvent", () => {

  // HACK: Needed due to mutually recursive types
  const DValue: t.Type<Value> = t.recursion("DValue", () => {
    return t.union([
      DValueNull,
      DValueInt,
      DValueFloat,
      DValueBool,
      DValueStr,
      t.type({ list: t.array(DValue) }),
      t.type({ obj: t.record(t.string, t.union([DValue, t.undefined])) }),
      DValueHtml,
    ]);
  });

  return t.union([
    t.type({ event: t.type({ event: t.string, args: t.union([DValue, t.null]) }) }),
    t.type({ coreResponse: t.type({ event: t.string, args: t.union([DValue, t.null]) }) }),
    t.type({ dialogFile: t.type({ event: t.string, title: OptionalString, file_kind: DDialogFileKind, filter_ext: t.array(t.string), create_dirs: t.boolean, }) }),
    t.type({ dialogEvent: t.type({ event: t.string, kind: t.union([DDialogEvtKind, t.null]), message: t.string, btn: t.union([DDialogBtn, t.null]), title: OptionalString, }) }),
    t.literal("preExit"),
    t.literal("postInit"),
  ]);
});