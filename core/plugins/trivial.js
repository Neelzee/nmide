window.plugins.set(
  "TrivialPlugin",
  {
    init: () => {
      return [];
    },
    update: (msg, model) => {
      return [];
    },
    view: model => {
      return {
        kind: "Frag",
        kids: [],
        text: null,
        attrs: [],
      }
    },
  }
);
