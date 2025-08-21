import { Eq, fromEquals } from "fp-ts/Eq";
import { Eq as StringEq } from "fp-ts/string";
import * as T from "fp-ts/Tuple";
import { ModuleUnknown as Nmlugin } from "./Module";


/**
 * State-Field equality check on field name.
 */
export const PartialStateFieldEq: Eq<[string, any]> =
  fromEquals(([x, _], [y, __]) => StringEq.equals(x, y))

export const NmluginEq: Eq<[string, Nmlugin]> = fromEquals(
  (x, y) => StringEq.equals(T.fst(x), T.fst(y))
);
