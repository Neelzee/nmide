import {
  HtmlBuilder,
  NmluginUnknown,
  tBool,
  THtml,
  tLookup,
  TMap,
  tObjLookup,
  tStr,
  TValueBool,
  TValueObj
} from "@nmide/js-utils";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import { pipe } from "fp-ts/lib/function";
import { fst } from "fp-ts/Tuple";

export const view = (model: TMap): THtml => {
  console.log("Model: ", model);
  return new HtmlBuilder()
    .kind("Div")
    .kids([
      pluginTab(model),
    ])
    .build();
}

const getPlugins = (): [string, NmluginUnknown][] => Array.from(
  //@ts-ignore
  window.plugins.entries()
);

const pluginTab = (model: TMap): THtml => {
  return new HtmlBuilder()
    .kind("Div")
    .attrs([
      { Class: "tab-plugin tab-content" }
    ])
    .kids(pipe(
      getPlugins(),
      A.map<[string, NmluginUnknown], string>(fst),
      A.filter(pln => pln !== "reggub"),
      A.map(pln => renderPlugin(getPluginState(model, pln))(pln)),
      A.prepend(new HtmlBuilder().kind("P").text("Plugins").build())
    ))
    .build();
}

const getPluginState = (model: TMap, pln: string): [boolean, boolean, boolean] => [
  pipe(
    model,
    tLookup<TValueObj>(`${pln}-state`),
    O.map<TValueObj, boolean>(el => pipe(
      el,
      tObjLookup<TValueBool>("toggle-init"),
      O.map<TValueBool, boolean>(b => b.Bool),
      O.getOrElse(() => false)
    )),
    O.getOrElse(() => false)
  ),
  pipe(
    model,
    tLookup<TValueObj>(`${pln}-state`),
    O.map<TValueObj, boolean>(el => pipe(
      el,
      tObjLookup<TValueBool>("toggle-update"),
      O.map<TValueBool, boolean>(b => b.Bool),
      O.getOrElse(() => false)
    )),
    O.getOrElse(() => false)
  ),
  pipe(
    model,
    tLookup<TValueObj>(`${pln}-state`),
    O.map<TValueObj, boolean>(el => pipe(
      el,
      tObjLookup<TValueBool>("toggle-view"),
      O.map<TValueBool, boolean>(b => b.Bool),
      O.getOrElse(() => false)
    )),
    O.getOrElse(() => false)
  ),
]

const renderPlugin = ([init, update, view]: [boolean, boolean, boolean]) => (pln: string): THtml => new HtmlBuilder()
  .kind("Div")
  .attrs([{ Class: "plugin" }])
  .kids([
    new HtmlBuilder()
      .kind("P")
      .text(pln),
    new HtmlBuilder()
      .kind("Label")
      .text("Init"),
    new HtmlBuilder()
      .kind("Input")
      .attrs([
        {
          OnClick: {
            Msg: ["toggle-init",
              {
                Obj: [["plugin", tStr(pln)],
                ["checked", tBool(init)]]
              }
            ]
          }
        },
        { Type: "checkbox" },
        { Checked: init }
      ]),
    new HtmlBuilder()
      .kind("Label")
      .text("Update"),
    new HtmlBuilder()
      .kind("Input")
      .attrs([
        {
          OnClick: {
            Msg: ["toggle-update",
              {
                Obj: [["plugin", tStr(pln)],
                ["checked", tBool(update)]]
              }
            ]
          }
        },
        { Type: "checkbox" },
        { Checked: update },
      ]),
    new HtmlBuilder()
      .kind("Label")
      .text("View"),
    new HtmlBuilder()
      .kind("Input")
      .attrs([
        {
          OnClick: {
            Msg: ["toggle-view",
              {
                Obj: [["plugin", tStr(pln)],
                ["checked", tBool(view)]]
              }
            ]
          }
        },
        { Type: "checkbox" },
        { Checked: view }
      ]),
  ])
  .build();
