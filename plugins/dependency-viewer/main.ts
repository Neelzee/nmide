import {
  HtmlBuilder,
  THtml,
  tLookup,
  TMap,
  TMsg,
  tObj,
  TValueBool,
  TValueObj,
  TValueStr
} from "@nmide/js-utils"
import MapBuilder from "@nmide/js-utils/lib/MapBuilder";
import { map, getOrElse } from "fp-ts/Option";
import { pipe } from "fp-ts/lib/function";

// @ts-ignore
window.plugins.set(
  "DependencyViewer",
  {
    init: (): TMap => {
      return new MapBuilder()
        .add("DependencyViewerInit", false)
        .build();
    },
    view: (model: TMap): THtml => {
      return new HtmlBuilder()
        .kids([
          new HtmlBuilder()
            .kind("button")
            .text("Render Dependency")
            .attrs([{
              onClick: {
                msg: ["dependency_render", {
                  str: pipe(
                    model,
                    tLookup<TValueStr>("info-module-graph"),
                    map<TValueStr, string>(a => a.str),
                    getOrElse(() => {
                      return "";
                    })
                  )
                }]
              }
            }])
        ])
        .build();
    },
    update: (msg: TMsg, model: TMap): TMap => {
      if (msg.msg[0] !== "dependency_render") return [];
      const skip = pipe(
        model,
        tLookup<TValueBool>("DependencyViewerInit"),
        map<TValueBool, boolean>(a => a.bool),
        getOrElse(() => false),
      );
      if (skip) return [];
      const graph = pipe(
        model,
        tLookup<TValueObj>("info-module-graph"),
        getOrElse(() => tObj([])),
      );
      const div = document.createElement("div");
      div.id = "graph";
      document.body.appendChild(div);
      //@ts-ignore
      window.plugins.get("DependencyViewerHelper").render(graph);
      return [];
    },
  }
);
