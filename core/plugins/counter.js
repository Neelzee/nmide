window.plugins.set(
  "COUNTER-PLUGIN",
  {
    init: () => {
      return [["counter", { "int": 0 }]];
    },
    update: (msg, model) => {
      if (msg.msg[0] === "increment") {
        const prevVal = model.find(([k, _]) => k === "counter")[1].Int;
        const increment = msg.msg[1]["int"];
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
        kind: "div",
        kids: [
          {
            kind: "text",
            kids: [],
            text: `counter: ${count === undefined ? "No Count found" : count[1]["int"]}`,
            attrs: [],
          },
          {
            kind: "button",
            kids: [],
            text: "click",
            attrs: [
              { "onClick": { "msg": ["increment", { "int": 1 }] } }
            ],
          }
        ],
        text: null,
        attrs: [],
      }
    },
  }
);
