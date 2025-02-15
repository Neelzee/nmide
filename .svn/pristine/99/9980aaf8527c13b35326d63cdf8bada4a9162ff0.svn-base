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

export interface AsyncNmluginUnknown {
  init: () => Promise<unknown>,
  view: (model: TMap) => Promise<unknown>;
  update: (msg: TMsg, model: TMap) => Promise<unknown>;
}

export interface AsyncNmluginVerified {
  init: () => Promise<TMap>,
  view: (model: TMap) => Promise<THtml>;
  update: (msg: TMsg, model: TMap) => Promise<TMap>;
}

export const toAsyncUnknown = (nmlugin: NmluginUnknown): AsyncNmluginUnknown => {
  return {
    init: () => new Promise(nmlugin.init),
    view: (model: TMap) => new Promise(() => nmlugin.view(model)),
    update: (msg: TMsg, model: TMap) => new Promise(() => nmlugin.update(msg, model)),
  };
}

export const toAsyncVerified = (nmlugin: NmluginVerified): AsyncNmluginVerified => {
  return {
    init: () => new Promise(nmlugin.init),
    view: (model: TMap) => new Promise(() => nmlugin.view(model)),
    update: (msg: TMsg, model: TMap) => new Promise(() => nmlugin.update(msg, model)),
  };
}
