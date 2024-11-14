window.plugins.set(
  "COUNTER-PLUGIN",
  {
    init: () => {
      return [["counter", { "Int": 0 }]];
    },
    update: (msg, model) => {
      if (msg.Msg[0] === "increment") {
        const prevVal = model[0][1]["Int"];
        const increment = msg.Msg[1]["Int"];
        return [
          ["counter",
            {
              "Int": prevVal === undefined
                ? 0
                : prevVal + (increment === undefined ? 1 : increment)
            }
          ]
        ];
      }
      return model;
    },
    view: model => {
      const count = model.find(([k, _]) => k === "counter");
      return {
        kind: "Div",
        kids: [
          {
            kind: "Text",
            kids: [],
            text: `counter: ${count === undefined ? "No Count found" : count[1]["Int"]}`,
            attrs: [],
          },
          {
            kind: "Button",
            kids: [],
            text: "Click",
            attrs: [
              { "OnClick": { "Msg": ["increment", { "Int": 1 }] } }
            ],
          }
        ],
        text: null,
        attrs: [],
      }
    },
  }
);
