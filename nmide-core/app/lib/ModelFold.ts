import { pipe } from "fp-ts/lib/function";
import * as S from "fp-ts/string";
import * as A from "fp-ts/Array";
import * as T from "fp-ts/Tuple";
import { TMap, TValue } from "nmide-js-utils/bindings/TMap";
import { Eq, fromEquals } from "fp-ts/Eq";
import { Ord } from "fp-ts/lib/Ord";

export const MapEq: Eq<[string, TValue]> =
  fromEquals(([x, _], [y, __]) => S.Eq.equals(x, y))

export const MapOrd: Ord<[string, any]> =
{
  compare: (x, y) => S.Ord.compare(T.fst(x), T.fst(y)),
  equals: (x, y) => S.Ord.equals(T.fst(x), T.fst(y))
};

const ModelFold = (xs: TMap, ys: TMap): TMap => pipe(
  A.difference(MapEq)(ys)(xs),
  A.union(MapEq)(ys),
)

export const ModelFoldPrev = (prevMap: TMap, newMap: TMap) => pipe(
  A.difference(MapEq)(prevMap)(newMap),
  A.concat(newMap)
);

export default ModelFold;
