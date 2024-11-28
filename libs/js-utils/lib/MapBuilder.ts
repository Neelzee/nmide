import { pipe } from "fp-ts/function";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import * as Eq from "fp-ts/Eq";
import { TMap, TValue } from "./TMap";
import {
  TMapPair,
  tValueMaybe,
  TValuePrimitive
} from "./Types";
import { GroupBy } from "./Utils";
import { TotalTMapFieldEq, TMapPartialEq } from "./Eq";

type MapBuilderProps = TValue | MapBuilder | TValuePrimitive;

export default class MapBuilder {
  private lst: [string, MapBuilderProps][] = [];

  public add(k: string, v: TValue | MapBuilder | TValuePrimitive) {
    this.lst.push([k, v]);
    return this;
  }

  public build(): TMap {
    const filterMapBuilder = (x: [string, MapBuilderProps]): x is [string, MapBuilder] =>
      x[1] instanceof MapBuilder;

    const filterTValue = (x: [string, MapBuilderProps]): x is [string, TValue] =>
      typeof x[1] === "object" && !Array.isArray(x[1]);

    const filterPrimitive = (x: [string, MapBuilderProps]): x is [string, TValuePrimitive] =>
      !filterMapBuilder(x) && !filterTValue(x);

    const lst = this.lst;

    this.lst = [];

    return pipe(
      [
        pipe(
          A.filter(filterMapBuilder)(lst),
          A.map<[string, MapBuilder], TMapPair>(([k, v]) => [k, { obj: v.build() }]),
        ),
        pipe(
          A.filter(filterPrimitive)(lst),
          A.filterMap(
            ([k, v]) => O.map<TValue, TMapPair>(_v => [k, _v])(tValueMaybe(v))
          ),
        ),
        pipe(
          A.filter(filterTValue)(lst),
        ),
      ],
      GroupBy(Eq.fromEquals(TMapPartialEq.equals)),
      A.filterMap(A.head),
      A.reduce<TMap, TMap>([], A.union(TotalTMapFieldEq)),
    );
  }
}


