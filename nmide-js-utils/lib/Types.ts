import { TValue } from "./TMap";
import * as O from "fp-ts/Option";

export type TValuePrimities = number
  | boolean
  | string
  | TValuePrimities[]
  | [string, TValuePrimities][];

export type TValueInt = { Int: number };
export const isTInt = (x: TValue): x is TValueInt => "Int" in x;
export type TValueFloat = { Float: number };
export const isTFloat = (x: TValue): x is TValueFloat => "Float" in x;
export type TValueStr = { Str: string };
export const isTStr = (x: TValue): x is TValueStr => "Str" in x;
export type TValueBool = { Bool: boolean };
export const isTBool = (x: TValue): x is TValueBool => "Bool" in x;
export type TValueList = { List: TValue[] };
export const isTList = (x: TValue): x is TValueList => "List" in x;
export type TValueObj = { Obj: [string, TValue][] };
export const isTObj = (x: TValue): x is TValueObj => "Obj" in x;

export const isTValue = (x: any): x is TValue => isTInt(x)
  || isTFloat(x)
  || isTStr(x)
  || isTBool(x)
  || isTList(x)
  || isTObj(x);

export const tInt = <T extends number = number>(n: T): TValueInt => {
  return { Int: n };
};

export const tFloat = (n: number): TValueFloat => {
  return { Float: n };
};

export const tStr = (s: string): TValueStr => {
  return { Str: s };
};

export const tBool = (s: boolean): TValueBool => {
  return { Bool: s };
};

export const tList = <T extends TValue>(lst: T[]): TValueList => {
  return { List: lst };
};

export const tObj = <T extends TValue>(obj: [string, T][]): TValueObj => {
  return { Obj: obj };
}

export const tValueMaybe = <T>(t: T): O.Option<TValue> => {
  if (typeof t === "number") {
    return O.some(t % 1 === 0 ? tInt(t) : tFloat(t));
  }
  if (typeof t === "boolean") {
    return O.some(tBool(t));
  }
  if (typeof t === "string") {
    return O.some(tStr(t));
  }
  if (Array.isArray(t)) {
    if (t.length === 0) {
      return O.some({ List: t })
    }
    if (Array.isArray(t[0]) && t[0].length === 2) {
      const [a, b] = t[0];
      if (typeof a === "string" && isTValue(b)) {
        return O.some({ Obj: t });
      }
    }
    return O.some({ List: t });
  }
  return O.none;
};

export type MaybeTValue = O.Option<TValue>;

export type TMapPair = [string, TValue];

export type MaybeTMapPair = O.Option<TMapPair>;
