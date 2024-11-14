// @ts-ignore
window.plugins.set("DependencyViewer", {
  init: function() {
    return [["init", { "Bool": false }]];
  },
  view: function(model) {
    return {
      kind: "Div",
      kids: [
        {
          kind: "Button",
          kids: [],
          text: "Render",
          attrs: [{ "OnClick": { "Msg": ["render", { "Int": 0 }] } }],
        },
      ],
      text: null,
      attrs: [],
    };
  },
  update: function(msg, model) {
    const [m, _] = msg.Msg;
    if (m !== "render") return [];
    const lookup = model.find(([k, _]) => k === "init");
    if (lookup !== undefined && lookup[1].Bool === false) {
      const div = document.createElement("div");
      div.id = "graph";
      document.body.appendChild(div);
      window.plugins.get("d3").render();
      return [["init", { "Bool": true }]];
    }
    return [];
  },
});
