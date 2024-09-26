import { TMap } from "./bindings/TMap";
import { TMsg } from "./bindings/TMsg";

type Nmlugin = {
  init: () => unknown,
  view: (model: TMap) => unknown,
  update: (msg: TMsg, model: TMap) => unknown,
};

export default Nmlugin;
