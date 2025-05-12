import type { Html, Value, Event } from "@nmide/js-utils";
import * as t from "io-ts";
import { DValueBool, DValueFloat, DValueInt, DValueNull, DValueStr } from "./value_decoder";
import { DDialogBtn, DDialogEvtKind, DDialogFileKind } from "./event_decoder";

export const OptionalString = t.union([t.string, t.null]);

export const DHtml: t.Type<Html> = t.recursion("DHtml", () => {

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
      t.type({ html: DHtml }),
    ]);
  });

  const DEvent: t.Type<Event> = t.union([
    t.type({ event: t.type({ event: t.string, args: t.union([DValue, t.null]) }) }),
    t.type({ coreResponse: t.type({ event: t.string, args: t.union([DValue, t.null]) }) }),
    t.type({ dialogFile: t.type({ event: t.string, title: OptionalString, file_kind: DDialogFileKind, filter_ext: t.array(t.string), create_dirs: t.boolean, }) }),
    t.type({ dialogEvent: t.type({ event: t.string, kind: t.union([DDialogEvtKind, t.null]), message: t.string, btn: t.union([DDialogBtn, t.null]), title: OptionalString, }) }),
    t.literal("nmide://pre-exit"),
    t.literal("nmide://post-init"),
  ]);


  const DAttr = t.union([
    t.type({ "id": t.string }),
    t.type({ "clss": t.string }),
    t.type({ "style": t.string }),
    t.type({ "click": DEvent }),
    t.type({ "onInput": DEvent }),
    t.type({ "emitInput": DEvent }),
    t.type({ "src": t.string }),
    t.type({ "type": t.string }),
    t.type({ "checked": t.boolean }),
    t.type({ custom: t.tuple([t.string, t.string]) })
  ]);

  const DHtmlBody = t.type({ kids: t.array(DHtml), attrs: t.array(DAttr), text: OptionalString, })

  return t.union([
    t.type({ div: DHtmlBody }),
    t.type({ h3: DHtmlBody }),
    t.type({ h1: DHtmlBody }),
    t.type({ h2: DHtmlBody }),
    t.type({ p: DHtmlBody }),
    t.type({ h4: DHtmlBody }),
    t.type({ h5: DHtmlBody }),
    t.type({ h6: DHtmlBody }),
    t.type({ span: DHtmlBody }),
    t.type({ section: DHtmlBody }),
    t.type({ article: DHtmlBody }),
    t.type({ aside: DHtmlBody }),
    t.type({ audio: DHtmlBody }),
    t.type({ b: DHtmlBody }),
    t.type({ br: DHtmlBody }),
    t.type({ button: DHtmlBody }),
    t.type({ code: DHtmlBody }),
    t.type({ em: DHtmlBody }),
    t.type({ fieldset: DHtmlBody }),
    t.type({ form: DHtmlBody }),
    t.type({ img: DHtmlBody }),
    t.type({ input: DHtmlBody }),
    t.type({ label: DHtmlBody }),
    t.type({ link: DHtmlBody }),
    t.type({ li: DHtmlBody }),
    t.type({ menu: DHtmlBody }),
    t.type({ nav: DHtmlBody }),
    t.type({ ol: DHtmlBody }),
    t.type({ option: DHtmlBody }),
    t.type({ script: DHtmlBody }),
    t.type({ select: DHtmlBody }),
    t.type({ style: DHtmlBody }),
    t.type({ svg: DHtmlBody }),
    t.type({ table: DHtmlBody }),
    t.type({ td: DHtmlBody }),
    t.type({ th: DHtmlBody }),
    t.type({ ul: DHtmlBody }),
    t.type({ video: DHtmlBody }),
    t.type({ frag: DHtmlBody }),
    t.type({ tr: DHtmlBody }),
    t.type({ tbody: DHtmlBody }),
    t.type({ main: DHtmlBody }),
    t.type({ textArea: DHtmlBody }),
  ]);
});
