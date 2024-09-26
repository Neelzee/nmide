import { useEffect } from "react";
import { TMap } from "./bindings/TMap";
import Nmlugin from "./Nmlugin";
import * as t from "io-ts";
import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import ModelFold from "./ModelFold";
import { TValue } from "./bindings/TValue";
import { pipe } from "fp-ts/lib/function";
import { PathReporter } from "io-ts/PathReporter";

const Value: t.RecursiveType<any, TValue> = t.recursion("Value", () =>
  t.union([
    t.type({ "Int": t.number }),
    t.type({ "Float": t.number }),
    t.type({ "Bool": t.boolean }),
    t.type({ "Str": t.string }),
    t.type({ "List": t.array(Value) }),
    t.type({ "Obj": t.array(t.tuple([t.string, Value])) }),
  ])
);

const Map = t.type({
  map: t.array(t.tuple([t.string, Value])),
});

const Init = (plugins: Nmlugin[], setModel: React.Dispatch<React.SetStateAction<TMap>>) => {
  useEffect(() => {
    setModel(
      pipe(
        plugins,
        A.map(
          p => pipe(
            p.init(),
            Map.decode,
            decoded => E.isRight(decoded)
              ? E.right(decoded.right)
              : E.left(new Error(`Failed to decode model: ${PathReporter.report(decoded).join("\n")}`)),
            E.getOrElse<Error, { map: [string, TValue][] }>(err => {
              console.error(err);
              return { map: [] };
            })
          )
        ),
        A.reduce({ map: [] }, ModelFold)
      )
    )
  }, [plugins]);
};

export default Init;
