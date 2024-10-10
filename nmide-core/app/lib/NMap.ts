import { TMap, TValue } from "./bindings/TMap";
import * as M from "fp-ts/Map";
import * as S from "fp-ts/string";
import * as A from "fp-ts/Array";
import * as SG from "fp-ts/Semigroup";

export type NMap = Map<string, TValue>;

export const FromTMap = (tmap: TMap): NMap =>
  M.fromFoldable(S.Eq, SG.first<TValue>(), A.Foldable)(tmap);

