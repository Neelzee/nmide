import {
  isTObj,
  isTStr,
  setTObjField,
  tBool,
  tLookup,
  tLookupOr,
  TMap,
  TMsg,
  tObj,
  tObjLookup,
  tStr,
  TValueBool,
  TValueObj,
  TValueStr
} from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";

export const update = (msg: TMsg, model: TMap): TMap => {
  const newModel: TMap = [];
  if (msg.Msg[0] === "reggub-tab-btn" && isTStr(msg.Msg[1])) {
    //@ts-ignore
    window.plugins.get("reggub_helper").openTab(msg.Msg[1].Str);
  }
  const hasInit = tLookupOr<TValueBool>("reggub-init")(tBool(false))(model).Bool;
  if (!hasInit) {
    newModel.push(["reggub-init", tBool(true)]);
  } if (
    msg.Msg[0] === "toggle-init"
    || msg.Msg[0] === "toggle-update"
    || msg.Msg[0] === "toggle-view"
  ) {
    const msg_obj = msg.Msg[1];
    if (!isTObj(msg_obj)) {
      return [];
    }
    const pluginName = O.getOrElse(() => tStr(""))(tObjLookup<TValueStr>("plugin")(msg_obj)).Str;
    const checked = O.getOrElse(() => tBool(false))(tObjLookup<TValueBool>("checked")(msg_obj));
    const field = `${pluginName}-state`;
    const obj = O.getOrElse(() => tObj([["toggle-init", false], ["toggle-view", false], ["toggle-update", false]]))(tLookup<TValueObj>(field)(model));
    checked.Bool = !checked.Bool;
    const newObj = setTObjField(msg.Msg[0], checked)(obj);
    newModel.push([field, newObj]);
  }

  if (!hasInit) {
    window.cleanup = pipe(
      window.cleanup,
      A.filter(([k, _]) => k !== "reggub"),
    );
  }

  return newModel;
}
