import { pipe } from "fp-ts/lib/function";
import { Value } from "./Value";
import * as O from "fp-ts/Option";
import * as A from "fp-ts/Array";
import type { Html } from "./Html";

export type ValuePrimitive = number
  | null
  | boolean
  | string
  | ValuePrimitive[]
  | { [key in string]?: Value };

export type ValueNull = "null";
export type ValueInt = { int: number };
export const isTInt = (x: object): x is ValueInt => "int" in x;
export type ValueFloat = { float: number };
export const isTFloat = (x: object): x is ValueFloat => "float" in x;
export type ValueStr = { str: string };
export const isTStr = (x: object): x is ValueStr => "str" in x;
export type ValueBool = { bool: boolean };
export const isTBool = (x: object): x is ValueBool => "bool" in x;
export type ValueList = { list: Value[] };
export const isTList = (x: object): x is ValueList => "list" in x;
export type ValueObj = { obj: { [key in string]?: Value } };
export const isTObj = (x: object): x is ValueObj => "obj" in x;

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

export const tObj = <T extends ValuePrimitive>(obj: [string, T][]): ValueObj => pipe(
  obj,
  A.filterMap(
    // { "obj": { [key in string]?: Value } }
    ([k, v]) => O.map<Value, [string, Value]>(_v => [k, _v])(tValueMaybe(v))
  ),
  fromEntries,
);

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
    const obj = {};
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
export const isObj = (x: unknown): x is [string, ValuePrimitive][] => {
  // is it a list?
  if (!Array.isArray(x)) return false;
  // has it any elements?
  if (x.length === 0) return false;
  // is the element a tuple?
  if (!Array.isArray(x[0]) && x[0].length === 2) return false;
  // is the first element of the tuple?
  return isStr(x[0][0]);
};
export const isList = (x: unknown): x is ValuePrimitive[] =>
  Array.isArray(x) && !isObj(x);

export const tValueMaybe = <T>(t: T): O.Option<Value> => {
  if (t === null) {
    return O.some("null")
  }
  if (isFloat(t)) {
    return O.some(tFloat(t));
  }
  if (isInt(t)) {
    return O.some(tInt(t));
  }
  if (isBool(t)) {
    return O.some(tBool(t));
  }
  if (isStr(t)) {
    return O.some(tStr(t));
  }
  if (isList(t)) {
    return pipe(
      t,
      A.filterMap<ValuePrimitive, Value>(tValueMaybe),
      list => list.length !== t.length ? O.none : O.some({ list })
    );
  }
  if (isObj(t)) {
    return pipe(
      t,
      A.filterMap<[string, ValuePrimitive], [string, Value]>(
        ([k, v]) => O.map<Value, [string, Value]>(_v => [k, _v])(tValueMaybe(v))
      ),
      obj => obj.length !== t.length ? O.none : O.some(fromEntries(obj))
    );
  }
  return O.none;
};