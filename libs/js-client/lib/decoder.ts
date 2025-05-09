import type { Html, Value, Attr, ValueObj, ValueSimple, Instruction } from "@nmide/js-utils";
import * as t from "io-ts";

export const DValue: t.RecursiveType<any, Value> = t.recursion("DValue", () => {
  return t.union([
    t.literal("null"),
    t.type({ int: t.number }),
    t.type({ float: t.number }),
    t.type({ bool: t.boolean }),
    t.type({ str: t.string }),
    t.type({ list: t.array(DValue) }),
    t.type({ obj: t.record(t.string, t.union([DValue, t.undefined])) }),
    t.type({ html: DHtml })
  ]);
});

const OptionalString = t.union([t.string, t.null]);

const DDialogEvtKind = t.union([t.literal("info"), t.literal("warning"), t.literal("error")]);
const DDialogFileKind = t.union([t.literal("singleFile"), t.literal("singleDir"), t.literal("multiFile"), t.literal("saveFile"), t.literal("multiDir")]);
const DDialogBtn = t.union([t.literal("ok"), t.literal("okCancel"), t.literal("yesNo"), t.type({ okCustom: t.string }), t.type({ okCancelCustom: t.tuple([t.string, t.string]) })]);

export const DEvent = t.union([
  t.type({ event: t.type({ event: t.string, args: t.union([DValue, t.null]) }) }),
  t.type({ coreResponse: t.type({ event: t.string, args: t.union([DValue, t.null]) }) }),
  t.type({ dialogFile: t.type({ event: t.string, title: OptionalString, file_kind: DDialogFileKind, filter_ext: t.array(t.string), create_dirs: t.boolean, }) }),
  t.type({ dialogEvent: t.type({ event: t.string, kind: t.union([DDialogEvtKind, t.null]), message: t.string, btn: t.union([DDialogBtn, t.null]), title: OptionalString, }) }),
  t.literal("preExit"),
  t.literal("postInit"),
]);

export const DAttr = t.union([
  t.type({ "id": t.string }),
  t.type({ "clss": t.string }),
  t.type({ "style": t.string }),
  t.type({ "click": DEvent }),
  t.type({ "onInput": DEvent }),
  t.type({ "emitInput": DEvent }),
  t.type({ "src": t.string }),
  t.type({ "type": t.string }),
  t.type({ "checked": t.boolean }),
]);

export const DState = t.record(t.string, DValue);

export const DHtml: t.Type<Html> = t.recursion("DHtml", () => {

  const DHtmlBody = t.type({ kids: t.array(DHtml), attrs: t.array(DAttr), text: OptionalString, })

  return t.union([
    t.type({ div: DHtmlBody }),
    t.type({h3:DHtmlBody}),
    t.type({h1:DHtmlBody}),
    t.type({h2:DHtmlBody}),
    t.type({p:DHtmlBody}), 
    t.type({h4:DHtmlBody}), 
    t.type({h5:DHtmlBody}), 
    t.type({h6:DHtmlBody}),
    t.type({span:DHtmlBody}),
    t.type({section:DHtmlBody}),
    t.type({article:DHtmlBody}),
    t.type({aside:DHtmlBody}),
    t.type({audio:DHtmlBody}),
    t.type({b:DHtmlBody}),
    t.type({br:DHtmlBody}),
    t.type({button:DHtmlBody}),
    t.type({code:DHtmlBody}),
    t.type({em:DHtmlBody}),
    t.type({fieldset:DHtmlBody}),
    t.type({form:DHtmlBody}),
    t.type({img:DHtmlBody}),
    t.type({input:DHtmlBody}),
    t.type({label:DHtmlBody}),
    t.type({link:DHtmlBody}),
    t.type({li:DHtmlBody}),
    t.type({menu:DHtmlBody}),
    t.type({nav:DHtmlBody}),
    t.type({ol:DHtmlBody}),
    t.type({option:DHtmlBody}),
    t.type({script:DHtmlBody}),
    t.type({select:DHtmlBody}),
    t.type({style:DHtmlBody}),
    t.type({svg:DHtmlBody}),
    t.type({table:DHtmlBody}),
    t.type({td:DHtmlBody}),
    t.type({th:DHtmlBody}),
    t.type({ul:DHtmlBody}),
    t.type({video:DHtmlBody}),
    t.type({frag:DHtmlBody}),
    t.type({tr:DHtmlBody}),
    t.type({tbody:DHtmlBody}),
    t.type({main:DHtmlBody}),
    t.type({textArea:DHtmlBody}),
  ]);
});

const DInstr = <T extends t.Mixed>(name: string, a: T): t.Type<Instruction<t.TypeOf<T>>> =>
  t.recursion(name, () =>
    t.union([
      t.literal("noOp"),
      t.type({ add: t.tuple([t.string, a]) }),
      t.type({ rem: t.tuple([t.string, a]) }),
      t.type({ then: t.tuple([DInstr(name, a), DInstr(name, a)]) }),
    ])
  );

const DInstrHtml: t.Type<Instruction<Html>> = DInstr("DInstrHtml", DHtml);
const DInstrString: t.Type<Instruction<string>> = DInstr("DInstrHtml", t.string);
const DInstrAttr: t.Type<Instruction<Attr>> = DInstr("DInstrAttr", DAttr);

export const DRenderEvent: t.Type<[Instruction<Html>, Instruction<string>, Instruction<Attr>]> 
  = t.tuple([DInstrHtml, DInstrString, DInstrAttr])