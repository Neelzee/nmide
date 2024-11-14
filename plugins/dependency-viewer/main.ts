import { getValue, HtmlBuilder, THtml, tList, tLookup, TMap, TMsg, tObj, TValueBool, TValueObj } from "@nmide/js-utils"
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
        .add(
          "DependencyViewerModel",
          tObj([
            [
              "links", tList([
                tObj([["source", "id1"], ["target", "id1"]]),
                tObj([["source", "id2"], ["target", "id1"]]),
              ])
            ],
            [
              "nodes", tList([
                tObj([["id", "id1"]]),
                tObj([["id", "id2"]]),
              ])
            ],
          ])
        )
    },
    view: (model: TMap): THtml => {
      return new HtmlBuilder()
        .kids([
          new HtmlBuilder()
            .kind("Button")
            .text("Render Dependency")
            .attrs([{ OnClick: { Msg: ["dependency_render", { Int: 0 }] } }])
        ])
        .build();
    },
    update: (msg: TMsg, model: TMap): TMap => {
      if (msg.Msg[0] !== "dependency_render") return [];
      const skip = pipe(
        model,
        tLookup<TValueBool>("DependencyViewerInit"),
        map<TValueBool, boolean>(a => a.Bool),
        getOrElse(() => false),
      );
      if (skip) return [];
      const graph = pipe(
        model,
        tLookup<TValueObj>("DependencyViewerModel"),
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
