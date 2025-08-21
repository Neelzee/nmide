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

export const getValue = (x: Value): ValuePrimitive => {
  if (x === "null") return null;
  if (isTList(x)) return A.map(getValue)(x.list);
  if (isTObj(x)) {
    return x.obj
  }
  if (isTInt(x)) return x.int;
  if (isTFloat(x)) return x.float;
  if (isTBool(x)) return x.bool;
  if (isTHtml(x)) return x.html;
  return x.str;
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