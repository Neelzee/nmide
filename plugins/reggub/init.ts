import {
  emptyHtml,
  tBool,
  tLookup,
  TMap,
  TMsg,
  tObj,
  tObjLookupOr,
  TValueBool,
  TValueObj
} from "@nmide/js-utils";
import "@nmide/js-utils";
import * as O from "fp-ts/Option";

export const _init = (model: TMap) => {
  window.plugins.forEach((v, k) => {
    if (k === "reggub") return;
    const obj = O.getOrElse(() => tObj([
      ["toggle-init", false],
      ["toggle-view", false],
      ["toggle-update", false]
    ]))(tLookup<TValueObj>(`${k}-state`)(model));
    const initDisabled =
      tObjLookupOr<TValueBool>("toggle-init")(tBool(false))(obj).Bool;
    v.init = initDisabled
      ? () => []
      : v.init;
    const viewDisabled =
      tObjLookupOr<TValueBool>("toggle-view")(tBool(false))(obj).Bool;
    v.view = viewDisabled
      ? (_: TMap) => emptyHtml()
      : v.view;
    const updateDisabled =
      tObjLookupOr<TValueBool>("toggle-update")(tBool(false))(obj).Bool;
    v.update = updateDisabled
      ? (_: TMsg, __: TMap) => []
      : v.update;
  });
}

export const init = (): TMap => {
  _init([]);
  return [["reggub-init", tBool(false)]];
};
