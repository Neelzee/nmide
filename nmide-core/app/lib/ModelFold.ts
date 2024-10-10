import { pipe } from "fp-ts/lib/function";
import { NMap } from "./NMap";
import * as S from "fp-ts/string";
import * as M from "fp-ts/Map";
import * as R from "fp-ts/Refinement";


const ModelFold = (a: NMap, b: NMap): NMap => pipe(
  M.difference(S.Eq)(a)(b),
  M.filter(R.id())
);

export default ModelFold;
