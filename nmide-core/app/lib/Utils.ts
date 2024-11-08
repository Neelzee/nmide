import * as E from "fp-ts/Either";
import * as O from "fp-ts/Option";
import * as Eq from "fp-ts/Eq";
import * as A from "fp-ts/Array";
import * as T from "fp-ts/Tuple";
import * as S from "fp-ts/string";
import * as B from "fp-ts/boolean";
import { Eq as NumberEq } from "fp-ts/number";
import { Eq as BooleanEq } from "fp-ts/boolean";
import * as NA from "fp-ts/NonEmptyArray";
import * as Ord from "fp-ts/Ord";
import { pipe } from "fp-ts/function";
import { TMap, TValue } from "./bindings/TMap";
import { Monoid } from "fp-ts/Monoid";

export const NmDebugLog = <V>(v: V): V => {
  console.debug("Debug: ", v);
  return v;
};

export const NmDebugLogMsg = (msg: string): <V>(v: V) => V => <V>(v: V): V => {
  console.debug(`${msg}: `, v);
  return v;
};

export const MapEq: Eq.Eq<[string, TValue]> =
  Eq.fromEquals(([x, _], [y, __]) => S.Eq.equals(x, y))

export const MapOrd: Ord.Ord<[string, any]> =
{
  compare: (x, y) => S.Ord.compare(T.fst(x), T.fst(y)),
  equals: (x, y) => S.Ord.equals(T.fst(x), T.fst(y))
};

export const TMapFieldEq: Eq.Eq<[string, TValue]> = Eq.fromEquals(
  ([xk, xv], [yk, yv]) => S.Eq.equals(xk, yk)
    ? TValueEq.equals(xv, yv)
    : false
);

export type JsonValues = string | number | boolean | JsonValues[] | [string, JsonValues][];

export const TMapEq: Eq.Eq<TMap> = Eq.fromEquals(
  (x, y) => pipe(
    A.zip(x)(y),
    A.foldMap(B.MonoidAll)(([a, b]) => TMapFieldEq.equals(a, b)),
  )
);

/**
 * Will return true if two models have the same field
 */
export const TMapPartialEq: Eq.Eq<TMap> = Eq.fromEquals(
  (x, y) => pipe(
    A.zip(x)(y),
    A.foldMap(B.MonoidAny)(([a, b]) => MapEq.equals(a, b)),
  )
);

export type TValueG<T, F extends boolean = false> = T extends string
  ? { Str: string }
  : T extends boolean
  ? { Bool: boolean }
  : T extends TValue[]
  ? { List: TValue[] }
  : T extends [string, TValue][]
  ? { Obj: [string, TValue][] }
  : TValueNumber<T, F>

export type TValueNumber<T, F extends boolean> = T extends number
  ? F extends true
  ? { Float: number }
  : { Int: number }
  : never;

export type TValueInt = TValueG<number>;
export const isInt = (x: TValue): x is TValueInt => "Int" in x;
export type TValueFloat = TValueG<number, true>;
export const isFloat = (x: TValue): x is TValueFloat => "Float" in x;
export type TValueStr = TValueG<string>;
export const isStr = (x: TValue): x is TValueStr => "Str" in x;
export type TValueBool = TValueG<boolean>;
export const isBool = (x: TValue): x is TValueBool => "Bool" in x;
export type TValueList = TValueG<TValue[]>;
export const isList = (x: TValue): x is TValueList => "List" in x;
export type TValueObj = TValueG<[string, TValue][]>;
export const isObj = (x: TValue): x is TValueObj => "Obj" in x;

export type TValueMList<T extends TValue> = T extends TValueInt
  ? { List: TValueInt[] }
  : T extends TValueFloat
  ? { List: TValueFloat[] }
  : T extends TValueStr
  ? { List: TValueStr[] }
  : T extends TValueBool
  ? { List: TValueBool[] }
  : T extends TValueMList<infer G>
  ? { List: TValueMList<G>[] }
  : T extends TValueList
  ? { List: TValueList[] }
  : T extends TValueObj
  ? { List: TValueObj[] }
  : never;

export const TValueEq: Eq.Eq<TValue> = Eq.fromEquals((x, y) => {
  if (isInt(x) && isInt(y)) {
    return NumberEq.equals(x.Int, y.Int);
  } else if (isStr(x) && isStr(y)) {
    return S.Eq.equals(x.Str, y.Str);
  } else if (isFloat(x) && isFloat(y)) {
    return NumberEq.equals(x.Float, y.Float);
  } else if (isBool(x) && isBool(y)) {
    return BooleanEq.equals(x.Bool, y.Bool);
  } else if (isList(x) && isList(y)) {
    return A.getEq(TValueEq).equals(x.List, y.List);
  } else if (isObj(x) && isObj(y)) {
    return A.getEq(TMapFieldEq).equals(Array.from(x.Obj.values()), Array.from(y.Obj.values()));
  }
  return false;
});

export const GetOrElse = <R>(t: R): ((v: E.Either<Error, R>) => R) => E.getOrElse<Error, R>(e => {
  console.error("Error: ", e);
  return t;
});

export const lookup = <K, V>(k: K): (xs: [K, V][]) => O.Option<V> =>
  (xs: [K, V][]): O.Option<V> => pipe(
    xs,
    A.findFirst(([ok, _]) => ok === k),
    O.map(T.snd)
  );

export const NmluginEq: Eq.Eq<[string, any]> = {
  equals: (x: [string, any], y: [string, any]) => S.Eq.equals(T.fst(x), T.fst(y))
};

export const TMapSort = A.sort(MapOrd)

export const TMapOrd: Ord.Ord<[string, TValue]> =
{
  compare: (x, y) => S.Ord.compare(T.fst(x), T.fst(y)),
  equals: TMapFieldEq.equals
};

export const PluginMonoid: Monoid<TMap> = { concat: (xs: TMap, ys: TMap) => TMapSort(A.concat(xs)(ys)), empty: [] };

export const GroupBy = <V>(eq: Eq.Eq<V>): (xs: Array<V>) => V[][] => {
  return (xs: Array<V>): V[][] => {
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

export const tInt = (n: number): TValueInt => {
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

export const StateUpdateHandler = (stateFrontend: [string, TMap][]) =>
  E.map<[string, TMap][], TMap>(
    stateBackend =>
      pipe(
        A.concat(stateFrontend)(stateBackend),
        GroupBy(
          Eq.fromEquals(([_, x], [__, y]) => TMapPartialEq.equals(x, y))
        ),
        A.reduce([[], []], foldPartition),
        ([newModel, collisions]) => {
          if (A.isNonEmpty(collisions)) console.debug("Collisions: ", collisions);
          return newModel;
        },
      )
  );

export const ModelOverwrite = (prevModel: TMap, newModel: TMap): TMap => pipe(
  A.union(MapEq)(prevModel)(newModel),
);

export const TMapToString = (model: TMap): string => pipe(
  model,
  A.foldMap(S.Monoid)(([k, v]) => `${k}: ${JSON.stringify(v)}`),
);

