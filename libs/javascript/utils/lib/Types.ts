import { pipe } from "fp-ts/lib/function";
import { type Value } from "./Value";
import * as O from "fp-ts/Option";
import * as A from "fp-ts/Array";
import * as E from "fp-ts/Either";
import type { Html } from "./Html";
import { isT } from "./Utils";
import { DCoreModification } from "@nmide/js-decoder-lib";

export type ValuePrimitive = number
  | null
  | boolean
  | string
  | ValuePrimitive[]
  | { [key in string]?: ValuePrimitive }
  | Html;

export type ValueNull = "null";
export type ValueInt = { int: number };
export const isTInt = (x: unknown): x is ValueInt =>
  typeof x === "object"
  && x !== null
  && "int" in x
  && typeof x.int === "number";
export type ValueFloat = { float: number };
// NOTE: This is not a good way of checking for floats
export const isTFloat = (x: unknown): x is ValueFloat =>
  typeof x === "object"
  && x !== null
  && "float" in x
  && typeof x.float === "number"
  && isFloat(x.float);

export type ValueStr = { str: string };
export const isTStr = (x: unknown): x is ValueStr => typeof x === "object" && x !== null && "str" in x;
export type ValueBool = { bool: boolean };
export const isTBool = (x: unknown): x is ValueBool =>
  typeof x === "object"
  && x !== null
  && "bool" in x
  && typeof x.bool === "boolean";
export type ValueList = { list: Value[] };
export const isTList = (x: unknown): x is ValueList => typeof x === "object" && x !== null && "list" in x;
export type ValueObj = { obj: { [key in string]?: Value } };
export const isTObj = (x: unknown): x is ValueObj => typeof x === "object" && x !== null && "obj" in x;
export type ValueHtml = { html: Html };
export const isTHtml = (x: unknown): x is ValueHtml => typeof x === "object" && x !== null && "html" in x;

export type ValueSimple = Exclude<Exclude<Exclude<Value, ValueObj>, { html: Html }>, ValueList>;

export const isValue = (x: unknown): x is Value => typeof x !== "object"
  ? false
  : x === null
    ? false
    : isTInt(x)
    || isTFloat(x)
    || isTStr(x)
    || isTBool(x)
    || isTList(x)
    || isTObj(x);

export const tNull = (): ValueNull => "null";

export const tInt = <T extends number = number>(n: T): ValueInt => {
  return { int: n };
};

export const tFloat = (n: number): ValueFloat => {
  return { float: n };
};

export const tStr = (s: string): ValueStr => {
  return { str: s };
};

export const tBool = (s: boolean): ValueBool => {
  return { bool: s };
};

export const tList = <T extends ValuePrimitive>(lst: T[]): ValueList => {
  return {
    list: pipe(
      lst,
      A.filterMap(tValueMaybe),
    )
  };
};

type InnerObject = { [key in string]?: Value };

export const tObjLst = <T extends ValuePrimitive>(obj: [string, T][]): ValueObj => pipe(
  obj,
  A.filterMap(
    // { "obj": { [key in string]?: Value } }
    ([k, v]) => O.map<Value, [string, Value]>(_v => [k, _v])(tValueMaybe(v))
  ),
  fromEntries,
);

export const tObj = (obj: Record<string, unknown>): ValueObj => pipe(
  obj,
  flatten,
  A.filterMap(
    ([k, v]) => O.map<Value, [string, Value]>(_v => [k, _v])(tValueMaybe(v))
  ),
  fromEntries,
);

const flatten = (rec: Record<string, unknown>): [string, unknown][] => pipe(
  rec,
  Object.keys,
  A.map(k => [k, rec[k]]),
)

const fromEntries = (xs: [string, Value][]): ValueObj => pipe(
  xs,
  A.foldMap({
    concat(x, y) {
      const obj = x;
      Object.keys(y).forEach(k => obj[k] = y[k]);
      return obj;
    },
    empty: {} as InnerObject,
  })(([f, v]): InnerObject => {
    const obj: Record<string, Value | undefined> = {};
    obj[f] = v;
    return obj;
  }),
  obj => { return { obj }; },
);

export const isFloat = (x: unknown): x is number => typeof x === "number"
  ? x % 1 !== 0
  : false;
export const isInt = (x: unknown): x is number =>
  typeof x === "number" && !isFloat(x);
export const isBool = (x: unknown): x is boolean => typeof x === "boolean";
export const isStr = (x: unknown): x is string => typeof x === "string";
export const isHtml = (x: unknown): x is Html => {
  if (typeof x !== "object") {
    return false;
  }
  return E.isLeft(DCoreModification.decode(x));
}
export const isList = (x: unknown): x is ValuePrimitive[] =>
  Array.isArray(x);

export const tValueMaybeOr = <T extends Value>(t: unknown) => (fallback: T): T => pipe(
  tValueMaybe(t),
  o => O.isSome(o) && isT<T>(o.value)
    ? o.value
    : fallback
)

export const tValueMaybe = (t: unknown): O.Option<Value> => {
  if (t === null || t === undefined) return O.none;
  if (t === "null") {
    return O.some("null")
  }
  if (isTFloat(t)) {
    return O.some(tFloat(isNaN(t.float) ? 0 : t.float));
  }
  if (isTInt(t)) {
    return O.some(tInt(isNaN(t.int) ? 0 : t.int));
  }
  if (isTBool(t)) {
    return O.some(t);
  }
  if (isTStr(t)) {
    return O.some(t);
  }
  if (isTList(t)) {
    return O.some(t);
  }
  if (isTObj(t)) {
    return O.some(t);
  }
  if (isTHtml(t)) {
    return O.some(t);
  }
  return O.none;
};

export const toValuePrimitive = (val: Value): ValuePrimitive => {
  if (val === "null") return null;
  if (isTFloat(val)) return val.float;
  if (isTInt(val)) return val.int;
  if (isTStr(val)) return val.str;
  if (isTBool(val)) return val.bool;
  if (isTList(val)) return val.list.map(toValuePrimitive);
  if (isTObj(val)) return Object.fromEntries(
    Object.entries(val.obj)
      .filter((x): x is [string, Value] => x[1] !== undefined)
      .map(([k, v]) => [k, toValuePrimitive(v)])
  );
  return val.html;
}

