import * as t from "io-ts";
import { TValue } from "./TMap";
import { THtml } from "./THtml";

export const DValue: t.RecursiveType<any, TValue> = t.recursion("DValue", () =>
  t.union([
    t.type({ "Int": t.number }),
    t.type({ "Float": t.number }),
    t.type({ "Bool": t.boolean }),
    t.type({ "Str": t.string }),
    t.type({ "List": t.array(DValue) }),
    t.type({ "Obj": t.array(t.tuple([t.string, DValue])) }),
  ])
);
export const DMsg = t.type({ "Msg": t.tuple([t.string, DValue]) });
export const DAttrs = t.union([t.type({ "Id": t.string }),
t.type({ "Class": t.string }),
t.type({ "Style": t.string }),
t.type({ "OnClick": DMsg }),
t.type({ "OnInput": DMsg }),
t.type({ "EmitInput": t.string }),
t.type({ "Src": t.string }),
t.type({ "Type": t.string }),
t.type({ "Checked": t.boolean }),
]);
export const DHtmlKind = t.union([
  t.literal("Div"), t.literal("P"), t.literal("H1"), t.literal("H2"),
  t.literal("H3"), t.literal("H4"), t.literal("H5"), t.literal("H6"),
  t.literal("Span"), t.literal("Section"), t.literal("Article"),
  t.literal("Aside"), t.literal("Audio"), t.literal("B"), t.literal("Br"),
  t.literal("Button"), t.literal("Code"), t.literal("Em"),
  t.literal("Fieldset"), t.literal("Form"), t.literal("Img"),
  t.literal("Input"), t.literal("Label"), t.literal("Link"), t.literal("Li"),
  t.literal("Menu"), t.literal("Nav"), t.literal("Ol"), t.literal("Option"),
  t.literal("Select"), t.literal("Style"), t.literal("Svg"), t.literal("Table"),
  t.literal("Td"), t.literal("Th"), t.literal("Ul"), t.literal("Video"),
  t.literal("Frag"), t.literal("Text"), t.literal("Script"),
  t.literal("Tr"),
  t.literal("Tbody"),
]);
export const DHtml: t.RecursiveType<any, THtml> = t.recursion("DHtml", () => t.type({
  kind: DHtmlKind,
  kids: t.array(DHtml),
  text: t.union([t.string, t.null]),
  attrs: t.array(DAttrs),
}));
export const DMap = t.array(t.tuple([t.string, DValue]));
export const DMapArr = t.array(DMap);
export const DUpdateDecoder = t.array(t.tuple([t.string, DMap]));
export const DInitDecoder = DUpdateDecoder;
export const DViewDecoder = t.array(t.tuple([t.string, DHtml]));
