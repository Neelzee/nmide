import * as E from "fp-ts/Either";
import * as O from "fp-ts/Option";
import * as A from "fp-ts/Array";
import * as T from "fp-ts/Tuple";
import * as NA from "fp-ts/NonEmptyArray";
import { pipe } from "fp-ts/function";
import type { State } from "./State";
import type { Value } from "./Value";
import {
  isTBool,
  isTFloat,
  isTHtml,
  isTInt,
  isTList,
  isTObj,
  isTStr,
  type ValueObj,
  type ValuePrimitive
} from "./Types";
import { Ord as StringOrd } from "fp-ts/string";
import type { Html } from "./Html";
import { HtmlBuilder } from "./HtmlBuilder";

export const GetOrElse = <R>(t: R): ((v: E.Either<Error, R>) => R) =>
  E.getOrElse<Error, R>(e => {
    console.error("Error: ", e);
    return t;
  });

export const lookup = <K, V>(k: K): (xs: [K, V][]) => O.Option<V> =>
  (xs: [K, V][]): O.Option<V> => pipe(
    xs,
    A.findFirst(([ok, _]) => ok === k),
    O.map(T.snd)
  );

export const tLookup = <T extends Value = Value>(k: string): ((xs: State) => O.Option<T>) =>
  (xs: State): O.Option<T> => pipe(
    xs,
    obj => obj[k] as T,
    x => x === undefined ? O.none : O.some(x),
  );

export const sLookup = <T extends Value = Value>(k: string) =>
  (xs: State) => pipe(
    xs[k],
    O.fromNullable,
    O.match(
      () => O.none,
      x => isT<T>(x) ? O.some(x) : O.none,
    )
  );

export const tLookupOr = <T extends Value = Value>(k: string) =>
  (def: T) =>
    (xs: State): T => pipe(
      xs,
      tLookup(k),
      O.match(
        () => def,
        x => x as T,
      ),
    );

export const tObjLookup = <T extends Value = Value>(k: string): ((o: ValueObj) => O.Option<T>) =>
  (o: ValueObj) => pipe(
    o.obj,
    obj => obj[k] as T,
    x => x === undefined ? O.none : O.some(x),
  );

export const tObjLookupNullable = <T extends Value = Value>(k: string): ((o: ValueObj) => T | null) =>
  (o: ValueObj) => pipe(
    o,
    tObjLookupUnd<T>(k),
    x => x === undefined ? null : x,
  );


export const tObjLookupUnd = <T extends Value = Value>(k: string): ((o: ValueObj) => T | undefined) =>
  (o: ValueObj) => pipe(
    o.obj,
    obj => obj[k] as T,
  );

const getType = (o: Value): keyof Exclude<Value, "null"> | "null" => {
  if ("null" === o) return "null";
  const key = Object.keys(o)[0];
  //@ts-ignore This is valid
  return key;
}

export const tObjLookupOrType = <T extends Value>(k: string) =>
  (def: T) =>
    (o: ValueObj): T => pipe(
      o,
      tObjLookup(k),
      O.match(
        () => def,
        x => getType(def) === getType(x)
          ? x as T
          : def,
      ),
    );

export const tObjLookupOr = <T extends Value = Value>(k: string) =>
  (def: T) =>
    (o: ValueObj) => pipe(
      o,
      tObjLookup(k),
      O.match(
        () => def,
        x => x as T,
      ),
    );


type ValuePrimitiveMap<T extends Value> = T extends "null"
  ? null
  : T extends { int: number }
  ? number
  : T extends { float: number }
  ? number
  : T extends { str: string }
  ? string
  : T extends { bool: boolean }
  ? boolean
  : T extends { list: Value[] }
  ? ValuePrimitive[]
  : T extends { html: Html }
  ? Html
  : T extends { obj: Record<string, Value | undefined> }
  ? { [key in string]?: ValuePrimitive }
  : never;

export const getValue = <T extends Value>(x: T): ValuePrimitiveMap<T> => {
  if (x === "null") return null as ValuePrimitiveMap<T>;
  if (isTList(x)) return A.map(getValue)(x.list) as ValuePrimitiveMap<T>;
  if (isTObj(x)) {
    return x.obj as ValuePrimitiveMap<T>;
  }
  if (isTInt(x)) return x.int as ValuePrimitiveMap<T>;
  if (isTFloat(x)) return x.float as ValuePrimitiveMap<T>;
  if (isTBool(x)) return x.bool as ValuePrimitiveMap<T>;
  if (isTHtml(x)) return x.html as ValuePrimitiveMap<T>;
  return x.str as ValuePrimitiveMap<T>;
}

export const isValueT = <T extends ValuePrimitive>(x: ValuePrimitive, f = false): x is T => {
  if (x === null) return true;
  if (typeof x === "number" && !f) return true;
  if (typeof x === "number") return true;
  if (typeof x === "string") return true;
  if (typeof x === "boolean") return true;
  if (Array.isArray(x)) return true;
  return typeof x === "object";
};

export const isT = <T extends Value>(x: Value): x is T => {
  if (x === "null") return true;
  if (isTInt(x)) return true;
  if (isTFloat(x)) return true;
  if (isTBool(x)) return true;
  if (isTStr(x)) return true;
  if (isTList(x)) return true;
  if (isTHtml(x)) return true;
  return isTObj(x);
};


export const objAdd = (o: ValueObj, field: string, val: Value): ValueObj => {
  const obj = o;
  obj.obj[field] = val;
  return obj;
};
