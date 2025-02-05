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
import { GroupBy, PartialTMapFieldOrd } from "./Utils";
import { THtml } from "./THtml";
import { TAttr } from "./TAttr";
import { TMsg } from "./TMsg";


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
  (x, y) => {
    if (x.length === 0 || y.length === 0) {
      return false;
    }
    return pipe(
      A.concat(x)(y),
      el => el,
      A.sort(PartialTMapFieldOrd),
      GroupBy(PartialTMapFieldEq),
      A.foldMap(MonoidAny)(z => A.size(z) != 1)
    );
  }
);

/**
 * Two TValues are only equal if they are of the same type and value
 */
export const TValueEq: Eq<TValue> = fromEquals(
  (x, y) => isTInt(x) && isTInt(y)
    ? NumberEq.equals(x.int, y.int)
    : isTFloat(x) && isTFloat(y)
      ? NumberEq.equals(x.float, y.float)
      : isTStr(x) && isTStr(y)
        ? StringEq.equals(x.str, y.str)
        : isTBool(x) && isTBool(y)
          ? BooleanEq.equals(x.bool, y.bool)
          : isTList(x) && isTList(y)
            ? A.getEq(TValueEq).equals(x.list, y.list)
            : isTObj(x) && isTObj(y)
              ? A.getEq(TotalTMapFieldEq).equals(
                Array.from(x.obj.values()), Array.from(y.obj.values())
              )
              : false
);

export const NmluginEq: Eq<[string, Nmlugin]> = fromEquals(
  (x, y) => StringEq.equals(T.fst(x), T.fst(y))
);

export const THtmlEq: Eq<THtml> = fromEquals(
  (
    {
      kind: xkind
      , kids: xkids
      , text: xtext
      , attrs: xattrs
    },
    {
      kind: ykind
      , kids: ykids
      , text: ytext
      , attrs: yattrs
    }) => StringEq.equals(xkind, ykind)
    && pipe(
      xkids,
      A.zip(ykids),
      A.foldMap(MonoidAll)(([a, b]) => THtmlEq.equals(a, b))
    )
    && (
      (xtext === null && ytext === null)
      || (xtext !== null && ytext !== null && StringEq.equals(xtext, ytext))
    )
    && pipe(
      xattrs,
      A.zip(yattrs),
      A.foldMap(MonoidAll)(([a, b]) => TAttrEq.equals(a, b))
    )
);

export const TAttrEq: Eq<TAttr> = fromEquals(
  (x, y) => ("id" in x && "id" in y && StringEq.equals(x.id, y.id))
    || ("class" in x && "class" in y && StringEq.equals(x.class, y.class))
    || ("style" in x && "style" in y && StringEq.equals(x.style, y.style))
    || ("type" in x && "type" in y && StringEq.equals(x.type, y.type))
    || ("checked" in x && "checked" in y && BooleanEq.equals(x.checked, y.checked))
    || ("onClick" in x && "onClick" in y && TMsgEq.equals(x.onClick, y.onClick))
    || ("onInput" in x && "onInput" in y && TMsgEq.equals(x.onInput, y.onInput))
    || ("emitInput" in x && "emitInput" in y && StringEq.equals(x.emitInput, y.emitInput))
    || ("src" in x && "src" in y && StringEq.equals(x.src, y.src))
);

export const TMsgEq: Eq<TMsg> = fromEquals(
  ({ msg: [xk, xv] }, { msg: [yk, yv] }) => StringEq.equals(xk, yk)
    && TValueEq.equals(xv, yv)
);
