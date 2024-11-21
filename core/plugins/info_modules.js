window.plugins.set(
  "info_modules_js",
  {
    init: () => {
      return [];
    },
    update: (_, __) => {
      return [];
    },
    view: _ => {
      return {
        kind: "Button",
        kids: [],
        text: "Load Magnolia",
        attrs: [{
          OnClick: { Msg: ["info-module-find-file", { Str: "/home/nmf/Documents/magnolia-src" }] }
        }],
      }
    },
  }
);
