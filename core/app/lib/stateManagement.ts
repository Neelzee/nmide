import {
  GroupBy,
  PartialTMapOrd,
  TMap,
  TMapPartialEq,
  TValue
} from "@nmide/js-utils";
import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import * as T from "fp-ts/Tuple";
import { pipe } from "fp-ts/lib/function";
import { fromEquals } from "fp-ts/Eq";
import { fromCompare } from "fp-ts/Ord";

export const stateHandler = (stateA: [string, TMap][]) =>
  E.map<[string, TMap][], [TMap, [string, TMap][]]>(
    stateB =>
      pipe(
        A.concat(stateA)(stateB),
        A.sort(
          fromCompare(
            (x: [string, TMap], y: [string, TMap]) =>
              PartialTMapOrd.compare(T.snd(x), T.snd(y))
          )
        ),
        GroupBy(
          fromEquals(([_, x], [__, y]) => TMapPartialEq.equals(x, y))
        ),
        A.reduce<[string, TMap][], [[string, TValue][], [string, TMap][]]>
          (
            [[], []],
            (acc, cur) => [
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
          ),
      )
  );
