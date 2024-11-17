import { useEffect } from "react";
import { TMsg } from "@nmide/js-utils";
import { listen } from "@tauri-apps/api/event";
import { NmDebugLogMsg } from "@nmide/js-utils/lib/Debug";

const MsgListener = (
  setMsg: React.Dispatch<React.SetStateAction<TMsg | undefined>>
) => {
  useEffect(() => {
    listen<TMsg>("msg", ({ payload }) => {
      setMsg(NmDebugLogMsg("Msg")(payload));
    });
  }, []);
}

export default MsgListener;
