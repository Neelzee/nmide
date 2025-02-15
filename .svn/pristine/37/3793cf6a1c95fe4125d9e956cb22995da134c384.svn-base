import * as t from "io-ts";
import { TValue } from "./TMap";
import { THtml } from "./THtml";
import { Node } from "./tree";

export const DValue: t.RecursiveType<any, TValue> = t.recursion("DValue", () =>
  t.union([
    t.type({ "int": t.number }),
    t.type({ "float": t.number }),
    t.type({ "bool": t.boolean }),
    t.type({ "str": t.string }),
    t.type({ "list": t.array(DValue) }),
    t.type({ "obj": t.array(t.tuple([t.string, DValue])) }),
  ])
);
export const DValueArr = t.array(DValue);
export const DMsg = t.type({ "msg": t.tuple([t.string, DValue]) });
export const DAttrs = t.union([t.type({ "id": t.string }),
t.type({ "class": t.string }),
t.type({ "style": t.string }),
t.type({ "onClick": DMsg }),
t.type({ "onInput": DMsg }),
t.type({ "emitInput": t.string }),
t.type({ "src": t.string }),
t.type({ "type": t.string }),
t.type({ "checked": t.boolean }),
]);
export const DHtmlKind = t.union([
  t.literal("div"), t.literal("p"), t.literal("h1"), t.literal("h2"),
  t.literal("h3"), t.literal("h4"), t.literal("h5"), t.literal("h6"),
  t.literal("span"), t.literal("section"), t.literal("article"),
  t.literal("aside"), t.literal("audio"), t.literal("b"), t.literal("br"),
  t.literal("button"), t.literal("code"), t.literal("em"),
  t.literal("fieldset"), t.literal("form"), t.literal("img"),
  t.literal("input"), t.literal("label"), t.literal("link"), t.literal("li"),
  t.literal("menu"), t.literal("nav"), t.literal("ol"), t.literal("option"),
  t.literal("select"), t.literal("style"), t.literal("svg"), t.literal("table"),
  t.literal("td"), t.literal("th"), t.literal("ul"), t.literal("video"),
  t.literal("frag"), t.literal("text"), t.literal("script"),
  t.literal("tr"),
  t.literal("tbody"),
  t.literal("main"),
]);
export const DHtml: t.RecursiveType<any, THtml> = t.recursion("DHtml",
  () => t.type({
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
export const DEvent = t.type({
  event: t.string,
  module: t.string,
});
const DNodeImpl = <T extends t.Mixed>(type: T): t.RecursiveType<any, Node<t.TypeOf<typeof type>>> => t.recursion(
  "DNodeImpl",
  () => t.type({
    id: t.string,
    kids: t.array(DNodeImpl<T>(type)),
  })
);
export const DNode = <T extends t.Mixed>(type: T) => t.intersection([
  DNodeImpl(type),
  type,
]);
export const DIns = <T extends t.Mixed>(type: T) => t.union([
  t.type({ node: DNode<T>(type) }, "DRemIns"),
  t.type({ node: DNode<T>(type), f: t.Function }, "RModIns")
]);
export const DEventHandler = t.type({
  handler: t.Function,
  module: t.string,
});
export const DInsArr = <T extends t.Mixed>(type: T) => t.array(DIns<T>(type));
export const DCoreModification = t.type({
  uiModifications: DInsArr(DHtml),
  stateModifications: DInsArr(DValue),
  eventModifications: DInsArr(DEvent),
  newEventHandlers: t.array(t.tuple([t.string, DEventHandler]))
});
