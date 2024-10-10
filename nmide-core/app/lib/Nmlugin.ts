import { TMap, TValue } from "./bindings/TMap";
import { TMsg } from "./bindings/TMsg";

interface Nmlugin {
  init: () => unknown;
  view: (model: TMap) => unknown;
  update: (msg: TMsg, model: TMap) => unknown;
};

export default Nmlugin;
