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
  const hasInit = tLookupOr<TValueBool>("reggub-init")(tBool(true))(model).Bool;
  if (!hasInit) {
    return new HtmlBuilder()
      .kids([
        new HtmlBuilder()
          .kind("Div")
          .attrs([{ Class: "tab" }])
          .kids([
            new HtmlBuilder()
              .kind("Button")
              .text("Plugin")
              .attrs([
                { OnClick: { Msg: ["reggub-tab-btn", tStr("Plugin")] } },
                { Class: "tablinks" },
              ]),
            new HtmlBuilder()
              .kind("Button")
              .text("State")
              .attrs([
                { OnClick: { Msg: ["reggub-tab-btn", tStr("State")] } },
                { Class: "tablinks" },
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
    .kind("Div")
    .attrs([
      { Class: "tabstate tabcontent" },
      { Id: "State" },
    ])
    .kids([renderTable(model)])
    .build();
}

const renderState = (model: TMap): THtml[] => {
  return pipe(
    model,
    A.map(([field, value]) => new HtmlBuilder()
      .kind("Table")
      .attrs([
        { Class: "statefield" }
      ])
      .kids([
        new HtmlBuilder()
          .kind("Tbody")
          .kids([
            new HtmlBuilder()
              .kind("Tr")
              .kids([
                new HtmlBuilder()
                  .kind("Th")
                  .text("Field"),
                new HtmlBuilder()
                  .kind("Th")
                  .text("Value"),
              ]),
            new HtmlBuilder()
              .kind("Tr")
              .kids([
                new HtmlBuilder()
                  .kind("Td")
                  .text(field),
                new HtmlBuilder()
                  .kind("Td")
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
      .kind("Table")
      .kids(pipe(
        [
          new HtmlBuilder()
            .kind("Tr")
            .kids([
              new HtmlBuilder()
                .kind("Th")
                .text("Object-Field"),
              new HtmlBuilder()
                .kind("Th")
                .text("Object-Value"),
            ]),
        ],
        A.concat(pipe(
          x.Obj,
          A.map(([y, z]) => new HtmlBuilder()
            .kind("Tr")
            .kids([
              new HtmlBuilder().kind("Td").text(y),
              new HtmlBuilder().kind("Td").kids([renderTValue(z)]),
            ])
          )
        ))
      ))
      .build();
  } else if (isTList(x)) {
    return new HtmlBuilder()
      .kind("Table")
      .kids(pipe(
        [
          new HtmlBuilder()
            .kind("Tr")
            .kids([
              new HtmlBuilder()
                .kind("Th")
                .text("Value"),
            ]),
        ],
        A.concat(pipe(
          x.List,
          A.map(y => new HtmlBuilder()
            .kind("Tr")
            .kids([
              new HtmlBuilder().kind("Td").kids([renderTValue(y)]),
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
    .kind("Div")
    .attrs([
      { Class: "tabplugin tabcontent" },
      { Id: "Plugin" },
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
        { Id: `${pln}-init` },
        { Type: "checkbox" },
        { Checked: init }
      ]),
    new HtmlBuilder()
      .kind("Label")
      .text("Update"),
    new HtmlBuilder()
      .kind("Input")
      .attrs([
        { Id: `${pln}-update` },
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


export const renderTable = (model: TMap): THtml => {
  return new HtmlBuilder()
    .kind("Table")
    .attrs([{ Id: "reggub-state-table" }])
    .kids(pipe(
      [
        new HtmlBuilder()
          .kind("Tr")
          .kids([
            new HtmlBuilder().kind("Th").text("Field"),
            new HtmlBuilder().kind("Th").text("Value"),
          ])
          .build()
      ],
      A.concat(model.map(e => renderRow(e).build())),
    ))
    .build();
};

const renderRow = ([field, value]: [string, TValue]): HtmlBuilder => new HtmlBuilder()
  .kind("Tr")
  .kids([
    new HtmlBuilder()
      .kind("Td")
      .attrs([{ Class: "state_field" }])
      .text(field)
      .build(),
    new HtmlBuilder()
      .kind("Td")
      .kids([renderValue(value)])
      .build(),
  ]);

const renderValue = (x: TValue): HtmlBuilder => {
  const y = getValue(x);
  if (isTObj(x) && isObj(y)) {
    return new HtmlBuilder()
      .kind("Table")
      .kids(pipe(
        [
          new HtmlBuilder()
            .kind("Tr")
            .kids([
              new HtmlBuilder().kind("Td").text("Field"),
              new HtmlBuilder().kind("Td").text("Value"),
            ]),
        ],
        A.concat(pipe(
          x.Obj,
          A.map(
            ([i, v]) => new HtmlBuilder()
              .kind("Tr")
              .kids([
                new HtmlBuilder().kind("Td").text(i),
                renderValue(v),
              ])
          ),
        )),
      ))
  } else if (isTList(x) && isList(y)) {
    return new HtmlBuilder()
      .kind("Table")
      .kids(pipe(
        [
          new HtmlBuilder()
            .kind("Tr")
            .kids([
              new HtmlBuilder().kind("Td").text("Index"),
              new HtmlBuilder().kind("Td").text("Value"),
            ]),
        ],
        A.concat(pipe(
          x.List,
          A.mapWithIndex(
            (i, v) => new HtmlBuilder()
              .kind("Tr")
              .kids([
                new HtmlBuilder().kind("Td").text(`Index-${i}`),
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
    .attrs([{ Class: `state_value state_${typeof x}` }]);
