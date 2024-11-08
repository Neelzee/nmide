import { THtml } from "./bindings/THtml";
import { TMap } from "./bindings/TMap";
import { TMsg } from "./bindings/TMsg";

interface NmluginUnknown {
  init: () => unknown;
  view: (model: TMap) => unknown;
  update: (msg: TMsg, model: TMap) => unknown;
};

export interface NmluginVerified {
  init: () => TMap;
  view: (model: TMap) => THtml;
  update: (msg: TMsg, model: TMap) => TMap;
};

export default NmluginUnknown;
