import { main } from "./index.js";
import { setup } from "./setup.js";

setup({
  cleanup: [],
  pluginAssets: [],
  root: document.body,
  client: (..._: any) => { },
  log: {
    info: console.log,
    error: console.error,
  },
  listen: (..._: any) => { },
  emit: (..._: any) => { },
});
main();
