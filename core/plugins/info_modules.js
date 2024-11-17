window.plugins.set(
  "info_modules_js",
  {
    init: () => {
      return [];
    },
    update: (msg, model) => {
      return [];
    },
    view: model => {
      return {
        kind: "Button",
        kids: [],
        text: "Render Magnolia",
        attrs: [{
          OnClick: { Msg: ["info-module-find-file", { Str: "/home/nmf/Documents/magnolia-src" }] }
        }],
      }
    },
  }
);
