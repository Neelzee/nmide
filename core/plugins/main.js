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
    if (m !== "dependency_render") return [];
    const lookup = model.find(([k, _]) => k === "init");
    if (lookup !== undefined && lookup[1].Bool === false) {
      const input = document.createElement("input");
      input.id = "info-module-input";
      document.appendChild(input);
      return [["init", { "Bool": true }]];
    }
    return [];
  },
});
