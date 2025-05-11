import type { Html, Value, ValueBool, ValueFloat, ValueInt, ValueNull, ValueStr } from "@nmide/js-utils";
import * as t from "io-ts";
import { DHtml } from "./html_decoder";

export const DValueNull: t.Type<ValueNull> = t.literal("null");
export const DValueInt: t.Type<ValueInt> = t.type({ int: t.number })
export const DValueFloat: t.Type<ValueFloat> = t.type({ float: t.number })
export const DValueBool: t.Type<ValueBool> = t.type({ bool: t.boolean })
export const DValueStr: t.Type<ValueStr> = t.type({ str: t.string });
export const DValueHtml: t.Type<{ html: Html }> = t.type({ html: DHtml });

export const DValue: t.Type<Value> = t.recursion("DValue", () => {
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

export const DState = t.record(t.string, DValue);