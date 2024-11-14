import { TMap } from "./TMap";
import { TMsg } from "./TMsg";
import { THtml } from "./THtml";

export interface NmluginUnknown {
  init: () => unknown;
  view: (model: TMap) => unknown;
  update: (msg: TMsg, model: TMap) => unknown;
};

export interface NmluginVerified {
  init: () => TMap;
  view: (model: TMap) => THtml;
  update: (msg: TMsg, model: TMap) => TMap;
};

