import { main } from "./index.js";
import { setup } from "./setup.js";
import * as E from "fp-ts/Either";

let app = null;

setup({
  root: document.body,
  client: (x?: any, ..._: any) => {
    if (x === undefined) return new Promise(r => r({}));
    if (x === "init") return new Promise(r => r(E.right([])));
    if (x === "view") return new Promise(r => r(E.right([])));
    if (x === "update") return new Promise(r => r(E.right([])));
  },
  listen: (x?: any, ...y: any) => new Promise(r => {
    console.log(x, ...y);
    if (x === "msg" && typeof y === "function" && app === null) {
      app = y;
      return r({});
    }
    if (x === "msg" && typeof y === "object") {
      // @ts-ignore
      app(y);
    }
    return r({});
  }),
  emit: (...x: any) => {
    console.log(x);
    return new Promise(r => r({}));
  },
  getPluginPaths: new Promise(r => r([
    "./plugins/ide-view.js",
    "./plugins/ide-framework.js",
    "./plugins/counter.js",
  ])),
});
main();
