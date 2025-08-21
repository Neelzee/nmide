import {
  getValue,
  HtmlBuilder,
  isList,
  isObj,
  isTList,
  isTObj,
  NmluginUnknown,
  tBool,
  THtml,
  tLookup,
  tLookupOr,
  TMap,
  tObjLookup,
  tStr,
  TValue,
  TValueBool,
  TValueObj,
} from "@nmide/js-utils";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import { pipe } from "fp-ts/lib/function";
import { fst } from "fp-ts/Tuple";
import { _init } from "./init";

export const view = (model: TMap): THtml => {
  _init(model);
  const hasInit = tLookupOr<TValueBool>("reggub-init")(tBool(true))(model).bool;
  if (!hasInit) {
    return new HtmlBuilder()
      .kids([
        new HtmlBuilder()
          .kind("div")
          .attrs([{ class: "tab" }])
          .kids([
            new HtmlBuilder()
              .kind("button")
              .text("Plugin")
              .attrs([
                { onClick: { msg: ["reggub-tab-btn", tStr("Plugin")] } },
                { class: "tablinks" },
              ]),
            new HtmlBuilder()
              .kind("button")
              .text("State")
              .attrs([
                { onClick: { msg: ["reggub-tab-btn", tStr("State")] } },
                { class: "tablinks" },
              ]),
          ]),
        // Tab-content
        new HtmlBuilder().kids([
          pluginTab(model),
          stateTab(model),
        ]),
      ])
      .build();
  }

  return new HtmlBuilder().build();
}

const stateTab = (model: TMap): THtml => {
  return new HtmlBuilder()
    .kind("div")
    .attrs([
      { class: "tabstate tabcontent" },
      { id: "State" },
    ])
    .kids([renderTable(model)])
    .build();
}

const renderState = (model: TMap): THtml[] => {
  return pipe(
    model,
    A.map(([field, value]) => new HtmlBuilder()
      .kind("table")
      .attrs([
        { class: "statefield" }
      ])
      .kids([
        new HtmlBuilder()
          .kind("tbody")
          .kids([
            new HtmlBuilder()
              .kind("tr")
              .kids([
                new HtmlBuilder()
                  .kind("th")
                  .text("Field"),
                new HtmlBuilder()
                  .kind("th")
                  .text("Value"),
              ]),
            new HtmlBuilder()
              .kind("tr")
              .kids([
                new HtmlBuilder()
                  .kind("td")
                  .text(field),
                new HtmlBuilder()
                  .kind("td")
                  .kids([renderTValue(value)])
              ]),
          ]),
      ])
      .build()
    ),
  );
}

const renderTValue = (x: TValue): THtml => {
  if (isTObj(x)) {
    return new HtmlBuilder()
      .kind("table")
      .kids(pipe(
        [
          new HtmlBuilder()
            .kind("tr")
            .kids([
              new HtmlBuilder()
                .kind("th")
                .text("Object-Field"),
              new HtmlBuilder()
                .kind("th")
                .text("Object-Value"),
            ]),
        ],
        A.concat(pipe(
          x.obj,
          A.map(([y, z]) => new HtmlBuilder()
            .kind("tr")
            .kids([
              new HtmlBuilder().kind("td").text(y),
              new HtmlBuilder().kind("td").kids([renderTValue(z)]),
            ])
          )
        ))
      ))
      .build();
  } else if (isTList(x)) {
    return new HtmlBuilder()
      .kind("table")
      .kids(pipe(
        [
          new HtmlBuilder()
            .kind("tr")
            .kids([
              new HtmlBuilder()
                .kind("th")
                .text("Value"),
            ]),
        ],
        A.concat(pipe(
          x.list,
          A.map(y => new HtmlBuilder()
            .kind("tr")
            .kids([
              new HtmlBuilder().kind("td").kids([renderTValue(y)]),
            ])
          )
        ))
      ))
      .build();
  }
  return new HtmlBuilder()
    .text(`${getValue(x)}`)
    .build();
}

const getPlugins = (): [string, NmluginUnknown][] => Array.from(
  //@ts-ignore
  window.plugins.entries()
);

const pluginTab = (model: TMap): THtml => {
  return new HtmlBuilder()
    .kind("div")
    .attrs([
      { class: "tabplugin tabcontent" },
      { id: "Plugin" },
    ])
    .kids(pipe(
      getPlugins(),
      A.map<[string, NmluginUnknown], string>(fst),
      A.filter(pln => pln !== "reggub"),
      A.map(pln => renderPlugin(getPluginState(model, pln))(pln)),
      A.prepend(new HtmlBuilder().kind("p").text("Plugins").build())
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
      O.map<TValueBool, boolean>(b => b.bool),
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
      O.map<TValueBool, boolean>(b => b.bool),
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
      O.map<TValueBool, boolean>(b => b.bool),
      O.getOrElse(() => false)
    )),
    O.getOrElse(() => false)
  ),
]

const renderPlugin = ([init, update, view]: [boolean, boolean, boolean]) => (pln: string): THtml => new HtmlBuilder()
  .kind("div")
  .attrs([{ class: "plugin" }])
  .kids([
    new HtmlBuilder()
      .kind("p")
      .text(pln),
    new HtmlBuilder()
      .kind("label")
      .text("Init"),
    new HtmlBuilder()
      .kind("input")
      .attrs([
        { id: `${pln}-init` },
        { type: "checkbox" },
        { checked: init }
      ]),
    new HtmlBuilder()
      .kind("label")
      .text("Update"),
    new HtmlBuilder()
      .kind("input")
      .attrs([
        { id: `${pln}-update` },
        { type: "checkbox" },
        { checked: update },
      ]),
    new HtmlBuilder()
      .kind("label")
      .text("View"),
    new HtmlBuilder()
      .kind("input")
      .attrs([
        { type: "checkbox" },
        { checked: view }
      ]),
  ])
  .build();


export const renderTable = (model: TMap): THtml => {
  return new HtmlBuilder()
    .kind("table")
    .attrs([{ id: "reggub-state-table" }])
    .kids(pipe(
      [
        new HtmlBuilder()
          .kind("tr")
          .kids([
            new HtmlBuilder().kind("th").text("Field"),
            new HtmlBuilder().kind("th").text("Value"),
          ])
          .build()
      ],
      A.concat(model.map(e => renderRow(e).build())),
    ))
    .build();
};

const renderRow = ([field, value]: [string, TValue]): HtmlBuilder => new HtmlBuilder()
  .kind("tr")
  .kids([
    new HtmlBuilder()
      .kind("td")
      .attrs([{ class: "state_field" }])
      .text(field)
      .build(),
    new HtmlBuilder()
      .kind("td")
      .kids([renderValue(value)])
      .build(),
  ]);

const renderValue = (x: TValue): HtmlBuilder => {
  const y = getValue(x);
  if (isTObj(x) && isObj(y)) {
    return new HtmlBuilder()
      .kind("table")
      .kids(pipe(
        [
          new HtmlBuilder()
            .kind("tr")
            .kids([
              new HtmlBuilder().kind("td").text("Field"),
              new HtmlBuilder().kind("td").text("Value"),
            ]),
        ],
        A.concat(pipe(
          x.obj,
          A.map(
            ([i, v]) => new HtmlBuilder()
              .kind("tr")
              .kids([
                new HtmlBuilder().kind("td").text(i),
                renderValue(v),
              ])
          ),
        )),
      ))
  } else if (isTList(x) && isList(y)) {
    return new HtmlBuilder()
      .kind("table")
      .kids(pipe(
        [
          new HtmlBuilder()
            .kind("tr")
            .kids([
              new HtmlBuilder().kind("td").text("Index"),
              new HtmlBuilder().kind("td").text("Value"),
            ]),
        ],
        A.concat(pipe(
          x.list,
          A.mapWithIndex(
            (i, v) => new HtmlBuilder()
              .kind("tr")
              .kids([
                new HtmlBuilder().kind("td").text(`Index-${i}`),
                renderValue(v),
              ])
          ),
        )),
      ),
      );
  } else if (!isObj(y) && !isList(y)) {
    return renderIntStrBool(y);
  }
  return new HtmlBuilder();
}

const renderIntStrBool = (x: number | boolean | string): HtmlBuilder =>
  new HtmlBuilder()
    .text(`${x}`)
    .attrs([{ class: `state_value state_${typeof x}` }]);
