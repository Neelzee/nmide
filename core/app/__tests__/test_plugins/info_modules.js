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
        kind: "button",
        kids: [],
        text: "Load Magnolia",
        attrs: [{
          onClick: { msg: ["info-module-find-file", { str: "/home/nmf/Documents/magnolia-src" }] }
        }],
      }
    },
  }
);
