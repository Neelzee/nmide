window.plugins.set(
  "TrivialPlugin",
  {
    init: () => {
      return [];
    },
    update: (_, __) => {
      return [];
    },
    view: _ => {
      return {
        kind: "frag",
        kids: [],
        text: null,
        attrs: [],
      }
    },
  }
);
