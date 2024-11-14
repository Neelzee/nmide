import { HtmlBuilder, tLookup, THtml, tList, TMap, TMsg, TValue, TValueList, getValue, isTStr, isValueT } from "@nmide/js-utils";
import MapBuilder from "@nmide/js-utils/lib/MapBuilder";
import { pipe } from "fp-ts/lib/function";
import * as O from "fp-ts/Option";
import * as A from "fp-ts/Array";

//@ts-ignore
windows.plugins.set(
  "plugin_installer",
  {
    init: (): TMap => {
      return new MapBuilder()
        .add("plugins", [])
        .build();
    },
    view: (model: TMap): THtml => {
      return new HtmlBuilder()
        .kids([
          new HtmlBuilder()
            .kind("Div")
            .kids(pipe(
              model,
              tLookup<TValueList>("plugins"),
              O.getOrElse(() => tList([])),
              v => v.List,
              A.map(getValue),
              A.filter(isValueT<string>),
              A.map(new HtmlBuilder().kind("P").text)
            ))
            .attrs([{ "Id": "plugin_container" }]),
        ])
        .build();
    },
    update: (msg: TMsg, model: TMap): TMap => {
      const [m, v] = msg.Msg;
      if (m !== "new_plugin") return [];
      return pipe(
        model,
        tLookup<TValueList>("plugins"),
        O.map<TValueList, TMap>(e => {
          e.List.push(v);
          return [["string", e]];
        }),
        O.getOrElse<TMap>(() => []),
      );
    },
  }
);
