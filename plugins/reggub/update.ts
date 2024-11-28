import {
  HtmlBuilder,
  isTObj,
  isTStr,
  setTObjField,
  tBool,
  tLookupOr,
  TMap,
  TMsg,
  tObj,
  tObjLookupOr,
  tStr,
  TValueBool,
  TValueObj,
  TValueStr
} from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import { Ord } from "fp-ts/lib/string";
import { toArray } from "fp-ts/lib/Map";

export const update = (msg: TMsg, model: TMap): TMap => {
  const newModel: TMap = [];
  if (msg.msg[0] === "reggub-tab-btn" && isTStr(msg.msg[1])) {
    //@ts-ignore
    window.plugins.get("reggub_helper").openTab(msg.msg[1].Str);
  }
  const hasInit = tLookupOr<TValueBool>("reggub-init")(tBool(false))(model).bool;
  if (!hasInit) {
    newModel.push(["reggub-init", tBool(true)]);
  } if (
    msg.msg[0] === "toggle-init"
    || msg.msg[0] === "toggle-update"
    || msg.msg[0] === "toggle-view"
  ) {
    const msg_obj = msg.msg[1];
    if (!isTObj(msg_obj)) {
      return newModel;
    }
    const pluginName = tObjLookupOr<TValueStr>("plugin")(tStr(""))(msg_obj).str;
    const checked = tObjLookupOr<TValueBool>("checked")(tBool(false))(msg_obj);
    const field = `${pluginName}-state`;
    const obj = tLookupOr<TValueObj>(field)(tObj([["toggle-init", false], ["toggle-view", false], ["toggle-update", false]]))(model);
    checked.bool = !checked.bool;
    const newObj = setTObjField(msg.msg[0], checked)(obj);
    newModel.push([field, newObj]);
  }

  if (!hasInit) {
    window.cleanup = pipe(
      window.cleanup,
      A.filter(([k, _]) => k !== "reggub"),
    );
    toArray(Ord)(window.plugins).forEach(([k, _]) => {
      const init = document.getElementById(`${k}-init`);
      const plugin = window.plugins.get(k);
      if (plugin === undefined) return;
      if (init instanceof HTMLInputElement) {
        init.addEventListener("click", () => {
          const initState = init.value !== "true";
          plugin.init = initState ? plugin.init : () => [];
        });
      }
      const _update = document.getElementById(`${k}-update`);
      if (_update instanceof HTMLInputElement) {
        _update.addEventListener("click", () => {
          const initState = _update.value !== "true";
          plugin.update = initState ? plugin.update : (_, __) => [];
        });
      }
      const view = document.getElementById(`${k}-view`);
      if (view instanceof HTMLInputElement) {
        view.addEventListener("click", () => {
          const initState = view.value !== "true";
          plugin.view = initState ? plugin.view : (_) => new HtmlBuilder().build();
        });
      }
    });
  }

  return newModel;
}
