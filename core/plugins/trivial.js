window.plugins.set("trivial", {
  init: async (_) => {
    return {
      uiModifications: [],
      stateModifications: [],
      eventModifications: [],
      newEventHandlers: [],
    };
  },
});
