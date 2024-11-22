import {
  isTObj,
  setTObjField,
  tBool,
  tLookup,
  TMap,
  TMsg,
  tObj,
  tObjLookup,
  tStr,
  TValueBool,
  TValueObj,
  TValueStr
} from "@nmide/js-utils";
import * as O from "fp-ts/Option";

export const update = (msg: TMsg, model: TMap): TMap => {
  if (
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
    console.log("New: ", [[field, newObj]]);
    return [[field, newObj]];
  }
  return [];
}
