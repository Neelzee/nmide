import { THtml } from "nmide-js-utils/bindings/THtml";
import { TMap } from "nmide-js-utils/bindings/TMap";
import { TMsg } from "nmide-js-utils/bindings/TMsg";

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
