import { pipe } from "fp-ts/lib/function";
import { TValue } from "./TMap";
import * as O from "fp-ts/Option";
import * as A from "fp-ts/Array";

export type TValuePrimitive = number
  | boolean
  | string
  | TValuePrimitive[]
  | [string, TValuePrimitive][];

export type TValueInt = { int: number };
export const isTInt = (x: object): x is TValueInt => "int" in x;
export type TValueFloat = { float: number };
export const isTFloat = (x: object): x is TValueFloat => "float" in x;
export type TValueStr = { str: string };
export const isTStr = (x: object): x is TValueStr => "str" in x;
export type TValueBool = { bool: boolean };
export const isTBool = (x: object): x is TValueBool => "bool" in x;
export type TValueList = { list: TValue[] };
export const isTList = (x: object): x is TValueList => "list" in x;
export type TValueObj = { obj: [string, TValue][] };
export const isTObj = (x: object): x is TValueObj => "obj" in x;

export const isTValue = (x: unknown): x is TValue => typeof x !== "object"
  ? false
  : x === null
    ? false
    : isTInt(x)
    || isTFloat(x)
    || isTStr(x)
    || isTBool(x)
    || isTList(x)
    || isTObj(x);

export const tInt = <T extends number = number>(n: T): TValueInt => {
  return { int: n };
};

export const tFloat = (n: number): TValueFloat => {
  return { float: n };
};

export const tStr = (s: string): TValueStr => {
  return { str: s };
};

export const tBool = (s: boolean): TValueBool => {
  return { bool: s };
};

export const tList = <T extends TValuePrimitive>(lst: T[]): TValueList => {
  return {
    list: pipe(
      lst,
      A.filterMap(tValueMaybe),
    )
  };
};

export const tObj = <T extends TValuePrimitive>(obj: [string, T][]): TValueObj => {
  return {
    obj: pipe(
      obj,
      A.filterMap(
        ([k, v]) => O.map<TValue, TMapPair>(_v => [k, _v])(tValueMaybe(v))
      )
    )
  };
}

export const isFloat = (x: unknown): x is number => typeof x === "number"
  ? x % 1 !== 0
  : false;
export const isInt = (x: unknown): x is number =>
  typeof x === "number" && !isFloat(x);
export const isBool = (x: unknown): x is boolean => typeof x === "boolean";
export const isStr = (x: unknown): x is string => typeof x === "string";
export const isObj = (x: unknown): x is [string, TValuePrimitive][] => {
  // is it a list?
  if (!Array.isArray(x)) return false;
  // has it any elements?
  if (x.length === 0) return false;
  // is the element a tuple?
  if (!Array.isArray(x[0]) && x[0].length === 2) return false;
  // is the first element of the tuple?
  return isStr(x[0][0]);
};
export const isList = (x: unknown): x is TValuePrimitive[] =>
  Array.isArray(x) && !isObj(x);

export const tValueMaybe = <T>(t: T): O.Option<TValue> => {
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
      A.filterMap<TValuePrimitive, TValue>(tValueMaybe),
      list => list.length !== t.length ? O.none : O.some({ list })
    );
  }
  if (isObj(t)) {
    return pipe(
      t,
      A.filterMap<[string, TValuePrimitive], [string, TValue]>(
        ([k, v]) => O.map<TValue, TMapPair>(_v => [k, _v])(tValueMaybe(v))
      ),
      obj => obj.length !== t.length ? O.none : O.some({ obj })
    );
  }
  return O.none;
};

export type MaybeTValue = O.Option<TValue>;

export type TMapPair = [string, TValue];

export type MaybeTMapPair = O.Option<TMapPair>;
