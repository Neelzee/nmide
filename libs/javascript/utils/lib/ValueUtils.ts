import { pipe } from "fp-ts/lib/function";
import { isTFloat, isTInt, isTList, isTObj, tFloat, tInt, tList, tObj, ValueObj, ValuePrimitive } from "./Types";
import { Value } from "./Value";
import * as A from "fp-ts/Array";
import { toValuePrimitive } from "./Types";

export const valueNanCheck = (v: Value): Value => {
  if (isTInt(v)) {
    const val = v.int;
    return tInt(isNaN(val) ? 0 : val);
  }
  if (isTFloat(v)) {
    const val = v.float;
    return tFloat(isNaN(val) ? 0 : val);
  }
  if (isTList(v)) {
    const val = v.list;
    return { list: val.map(valueNanCheck) };
  }
  if (isTObj(v)) {
    const val = Object.entries(recFlatten(v));
    const checkedValues: [string, ValuePrimitive][] = val
      .map(([k, v]): [string, Value] => [k, valueNanCheck(v)])
      .map(([k, v]) => [k, toValuePrimitive(v)]);
    const midObj: Record<string, ValuePrimitive> = Object.fromEntries(checkedValues);
    return unflatten(midObj);
  }
  return v
}

type TObjFlat = Record<string, Exclude<Value, ValueObj>>;

const recFlatten = ({ obj }: ValueObj): TObjFlat => pipe(
  obj,
  Object.entries<Value | undefined>,
  A.filter((x): x is [string, Value] => x[1] !== undefined),
  A.flatMap<[string, Value], [string, Exclude<Value, ValueObj>]>(([k, v]) => {
    const val: [string, Exclude<Value, ValueObj>][] = isTObj(v)
      ? Object.entries(recFlatten(v))
        .map(([nk, nv]) => [`${k}.${nk}`, nv])
      : [[k, v]];
    return val;
  }),
  Object.fromEntries
);


const unflatten = (flatObj: Record<string, ValuePrimitive>): ValueObj => {
  const obj = {};

  for (const [flatKey, value] of Object.entries(flatObj)) {
    const keys = flatKey.split('.');
    let current = obj;

    keys.forEach((key, idx) => {
      if (idx === keys.length - 1) {
        current[key] = value;
      } else {
        if (!(key in current) || typeof current[key] !== 'object') {
          current[key] = {};
        }
        current = current[key];
      }
    });
  }

  return tObj(obj);
};
