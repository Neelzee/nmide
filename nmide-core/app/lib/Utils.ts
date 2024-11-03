import * as E from "fp-ts/Either";
import * as O from "fp-ts/Option";
import * as Eq from "fp-ts/Eq";
import * as A from "fp-ts/Array";
import * as T from "fp-ts/Tuple";
import * as S from "fp-ts/string";
import { pipe } from "fp-ts/lib/function";
import Nmlugin from "./Nmlugin";
import { Monoid } from "fp-ts/lib/Monoid";
import { TMap } from "./bindings/TMap";
import ModelFold from "./ModelFold";

export const NmDebugLog = <T>(t: T): T => {
  console.debug("Debug: ", t);
  return t;
};

export const NmDebugLogMsg = (msg: string): <T>(t: T) => T => <T>(t: T): T => {
  console.debug(`${msg}: `, t);
  return t;
};

export const GetOrElse = <R>(t: R): ((v: E.Either<Error, R>) => R) => E.getOrElse<Error, R>(e => {
  console.error("Error: ", e);
  return t;
});

export const lookup = <K, V>(k: K): (xs: [K, V][]) => O.Option<V> =>
  (xs: [K, V][]): O.Option<V> => pipe(
    xs,
    A.findFirst(([ok, _]) => ok === k),
    O.map(T.snd)
  );

export const NmluginEq: Eq.Eq<[string, Nmlugin]> = {
  equals: (x: [string, Nmlugin], y: [string, Nmlugin]) => S.Eq.equals(T.fst(x), T.fst(y))
};

export const PluginMonoid: Monoid<TMap> = { concat: ModelFold, empty: [] };
