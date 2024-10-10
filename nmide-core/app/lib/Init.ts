import Nmlugin from "./Nmlugin";
import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import * as M from "fp-ts/Map";
import * as S from "fp-ts/string";
import * as SG from "fp-ts/Semigroup";
import ModelFold from "./ModelFold";
import { TValue } from "./bindings/TMap";
import { pipe } from "fp-ts/lib/function";
import { PathReporter } from "io-ts/PathReporter";
import { Monoid } from "fp-ts/lib/Monoid";
import { NMap } from "./NMap";
import { DMap } from "./Decoder";
import { invoke } from "@tauri-apps/api/core";

const pluginMonoid: Monoid<NMap> = { concat: ModelFold, empty: new Map() };
const pluginInit = (p: Nmlugin): NMap => pipe(
  p.init(),
  DMap.decode,
  decoded => E.isRight(decoded)
    ? E.right(M.fromFoldable(S.Eq, SG.first<TValue>(), A.Foldable)(decoded.right))
    : E.left(new Error(`Failed to decode model: ${PathReporter.report(decoded).join("\n")}`)),
  E.getOrElse<Error, NMap>(err => {
    console.error(err);
    return new Map();
  })
);

const Init = async (plugins: Nmlugin[]): Promise<E.Either<Error, null>> => invoke(
  "init",
  {
    tmodel: pipe(
      plugins,
      A.foldMap(pluginMonoid)(pluginInit),
      M.toArray(S.Ord)
    )
  }
).then(_ => E.right(null))
  .catch(err => E.left(err));

export default Init;
