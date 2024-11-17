import { Eq, fromEquals } from "fp-ts/Eq";
import { Eq as StringEq } from "fp-ts/string";
import { TValue, TMap } from "./TMap";
import { pipe } from "fp-ts/function";
import * as A from "fp-ts/Array";
import * as T from "fp-ts/Tuple";
import { MonoidAll, MonoidAny } from "fp-ts/boolean";
import { Eq as NumberEq } from "fp-ts/number";
import { Eq as BooleanEq } from "fp-ts/boolean";
import {
  isTBool,
  isTFloat,
  isTInt,
  isTList,
  isTObj,
  isTStr,
} from "./Types";
import { NmluginUnknown as Nmlugin } from "./Nmlugin";


/**
 * TMap-Field equality check on field name.
 */
export const PartialTMapFieldEq: Eq<[string, any]> =
  fromEquals(([x, _], [y, __]) => StringEq.equals(x, y))

/**
 * TMap-Field equality check.
 *
 * Given two fields, A and B:
 *
 * - If A.field_name != B.field_name, then false
 * - Else, if A.field_type != B.field_type, then false,
 * - Else, true
 */
export const TotalTMapFieldEq: Eq<[string, TValue]> = fromEquals(
  ([xk, xv], [yk, yv]) => StringEq.equals(xk, yk)
    ? TValueEq.equals(xv, yv)
    : false
);

/**
 * TMap Equality check, two TMaps, A and B are equal if and only if all fields
 * totally equal.
 */
export const TotalTMapEq: Eq<TMap> = fromEquals(
  (x, y) => pipe(
    A.zip(x)(y),
    A.foldMap(MonoidAll)(([a, b]) => TotalTMapFieldEq.equals(a, b)),
  )
);

/**
 * Will return true if two models have the same field
 */
export const TMapPartialEq: Eq<TMap> = fromEquals(
  (x, y) => pipe(
    A.zip(x)(y),
    A.foldMap(MonoidAny)(([a, b]) => PartialTMapFieldEq.equals(a, b)),
  )
);

/**
 * Two TValues are only equal if they are of the same type and value
 */
export const TValueEq: Eq<TValue> = fromEquals(
  (x, y) => isTInt(x) && isTInt(y)
    ? NumberEq.equals(x.Int, y.Int)
    : isTFloat(x) && isTFloat(y)
      ? NumberEq.equals(x.Float, y.Float)
      : isTStr(x) && isTStr(y)
        ? StringEq.equals(x.Str, y.Str)
        : isTBool(x) && isTBool(y)
          ? BooleanEq.equals(x.Bool, y.Bool)
          : isTList(x) && isTList(y)
            ? A.getEq(TValueEq).equals(x.List, y.List)
            : isTObj(x) && isTObj(y)
              ? A.getEq(TotalTMapFieldEq).equals(
                Array.from(x.Obj.values()), Array.from(y.Obj.values())
              )
              : false
);

export const NmluginEq: Eq<[string, Nmlugin]> = fromEquals(
  (x, y) => StringEq.equals(T.fst(x), T.fst(y))
);

