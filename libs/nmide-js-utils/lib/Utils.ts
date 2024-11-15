import * as E from "fp-ts/Either";
import * as O from "fp-ts/Option";
import { Eq, fromEquals } from "fp-ts/Eq";
import * as A from "fp-ts/Array";
import * as T from "fp-ts/Tuple";
import * as NA from "fp-ts/NonEmptyArray";
import { pipe } from "fp-ts/function";
import { TMap, TValue } from "./TMap";
import { Monoid } from "fp-ts/Monoid";
import { isObj, isTBool, isTFloat, isTInt, isTList, isTObj, isTStr, TMapPair, tObj, TValueBool, TValueFloat, TValueInt, TValueList, TValueObj, TValuePrimities, TValueStr } from "./Types";
import { PartialTMapFieldEq, TMapPartialEq } from "./Eq";
import { fromCompare, Ord } from "fp-ts/lib/Ord";
import { Ord as StringOrd } from "fp-ts/string";

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

export const tLookup = <T extends TValue = TValue>(k: string): ((xs: TMap) => O.Option<T>) =>
  (xs: TMap): O.Option<T> => pipe(
    xs,
    A.findFirst(([ok, _]) => ok === k),
    O.map(T.snd),
    O.match(
      () => O.none,
      el => isT<T>(el) ? O.some(el) : O.none,
    ),
  );

export const getValue = (x: TValue): TValuePrimities => {
  if (isTList(x)) return A.map(getValue)(x.List);
  if (isTObj(x)) {
    return A.map(([k, v]: TMapPair) => [k, getValue(v)])(x.Obj);
  }
  if (isTInt(x)) return x.Int;
  if (isTFloat(x)) return x.Float;
  if (isTBool(x)) return x.Bool;
  return x.Str;
}

export const isValueT = <T extends TValuePrimities>(x: TValuePrimities, f = false): x is T => {
  if (typeof x === "number" && !f) return true;
  if (typeof x === "number") return true;
  if (typeof x === "string") return true;
  if (typeof x === "boolean") return true;
  if (Array.isArray(x)) return true;
  return false;
};

export const isT = <T extends TValue>(x: TValue): x is T => {
  if (isTInt(x)) return true;
  if (isTFloat(x)) return true;
  if (isTBool(x)) return true;
  if (isTStr(x)) return true;
  if (isTList(x)) return true;
  return isTObj(x);
};

export const PluginMonoid: Monoid<TMap> = {
  concat: (xs: TMap, ys: TMap) => A.sort(PartialTMapFieldOrd)(A.concat(xs)(ys)),
  empty: []
};

/**
 * @example
 * ```haskell
 * groupBy :: (a -> a -> Bool) -> [a] -> [[a]]
 * groupBy _ [] = []
 * groupBy eq (x:xs) = (x:ys) : groupBy eq zs
 *                     where (ys, zs) = span (eq x) xs
 * ```
 */
export const GroupBy = <V>(eq: Eq<V>): (xs: V[]) => V[][] => {
  return (xs: V[]): V[][] => {
    if (A.isNonEmpty(xs)) {
      return pipe(
        NA.head(xs),
        x => {
          const spanXs = A.spanLeft((y: V) => eq.equals(x, y))(NA.tail(xs));
          const ys = spanXs.init;
          const zs = spanXs.rest;
          const foo = A.prepend<V[]>(A.prepend<V>(x)(ys))(GroupBy(eq)(zs));
          return foo;
        }
      )
    } else {
      return [];
    }
  }
}

/**
 * @example
 * ```haskell
 * foldPartition :: (TMap, [(String, TMap)]) -> [(String, TMap)] -> (TMap, [(String, TMap)])
 * foldPartition acc cur = (map snd (head cur) : fst acc, tail cur : snd acc)
 * ```
 */
const foldPartition = (acc: [TMap, [string, TMap][]], cur: [string, TMap][]): [TMap, [string, TMap][]] => pipe(
  [
    pipe(
      cur,
      A.head,
      O.map(T.snd),
      O.getOrElse<TMap>(() => []),
      A.concat(T.fst(acc))
    ),
    pipe(
      cur,
      A.tail,
      O.getOrElse<[string, TMap][]>(() => []),
      A.concat(T.snd(acc))
    )
  ]
);

export const PartialTMapFieldOrd: Ord<[string, TValue]>
  = fromCompare((x, y) => StringOrd.compare(T.fst(x), T.fst(y)))

export const PartialTMapOrd: Ord<TMap> = A.getOrd(PartialTMapFieldOrd);

/**
 * @example
 * ```haskell
 * stateUpdateHandler :: [(String, TValue)] -> [(String, TValue)] -> [(String, TValue)]
 * stateUpdateHandler sf sb = map foldPartition (group (sf ++ sb))
 * ```
 */
export const StateUpdateHandler = (stateFrontend: [string, TMap][]) =>
  E.map<[string, TMap][], [TMap, [string, TMap][]]>(
    stateBackend =>
      pipe(
        A.concat(stateFrontend)(stateBackend),
        A.sort(
          fromCompare(
            (x: [string, TMap], y: [string, TMap]) =>
              PartialTMapOrd.compare(T.snd(x), T.snd(y))
          )
        ),
        GroupBy(
          fromEquals(([_, x], [__, y]) => TMapPartialEq.equals(x, y))
        ),
        A.reduce([[], []], foldPartition),
      )
  );

/**
 * Takes union of two TMaps, is *NOT* associative. Left side takes precedence on
 * partial equality. I.e if one field exist in both maps, left-hand-side values
 * are kept.
 */
export const ModelOverwrite = (
  prevModel: TMap,
  newModel: TMap
): TMap => A.union(PartialTMapFieldEq)(prevModel)(newModel);

