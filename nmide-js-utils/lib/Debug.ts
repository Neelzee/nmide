import { pipe } from "fp-ts/function";
import * as A from "fp-ts/Array";
import *  as S from "fp-ts/string";
import { TMap } from "./TMap";

export const NmDebugLog = <T>(t: T): T => {
  console.debug("Debug: ", t);
  return t;
};

export const NmDebugLogMsg = (msg: string): <T>(t: T) => T => <T>(t: T): T => {
  console.debug(`${msg}: `, t);
  return t;
};

export const TMapToString = (model: TMap): string => pipe(
  model,
  A.foldMap(S.Monoid)(([k, v]) => `${k}: ${JSON.stringify(v)}`),
);
