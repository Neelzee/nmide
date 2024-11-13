import { pipe } from "fp-ts/function";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import * as Eq from "fp-ts/Eq";
import { TMap, TValue } from "./TMap";
import {
  isTValue,
  MaybeTMapPair,
  TMapPair,
  tObj,
  tValueMaybe,
  TValuePrimities
} from "./Types";
import { GroupBy, TMapFieldEq, TMapPartialEq } from "./Utils";

export default class MapBuilder {
  private lst: [string, TValue | MapBuilder | TValuePrimities][] = [];

  public add(k: string, v: TValue | MapBuilder | TValuePrimities) {
    this.lst.push([k, v]);
    return this;
  }

  public build(): TMap {
    const mapPrimitives
      = ([k, v]) => O.map<TValue, TMapPair>(_v => [k, _v])(tValueMaybe(v));

    const mapBuilders: ([k, v]: [string, any]) => MaybeTMapPair
      = ([k, v]) => v instanceof MapBuilder
        ? O.some([k, tObj(v.build())])
        : O.none;

    const filterTValues: ([k, v]: [string, any]) => MaybeTMapPair
      = ([k, v]) => isTValue(v)
        ? O.some([k, v])
        : O.none;

    const lst = this.lst;

    this.lst = [];

    return pipe(
      [
        A.filterMap(mapBuilders)(lst),
        A.filterMap(mapPrimitives)(lst),
        A.filterMap(filterTValues)(lst),
      ],
      GroupBy(Eq.fromEquals(TMapPartialEq.equals)),
      A.filterMap(A.head),
      A.reduce<TMap, TMap>([], A.union(TMapFieldEq)),
    );
  }
}


