const module = "CounterPlugin";
window.__nmideConfig__.modules.set(module, {
  name: module,
  init: async (core) => {
    await core.registerHandler(module, "counter", null)
      .catch((err) => console.error("error from module: ", err));
    return { state: "noOp", ui: ["noOp", "noOp", "noOp"] };
  },
  handler: async (event, __) => {
    return { state: "noOp", ui: ["noOp", "noOp", "noOp"] };
  }
});
