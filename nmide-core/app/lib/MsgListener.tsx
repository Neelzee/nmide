import { useEffect } from "react";
import { TMsg } from "./bindings/TMsg";
import { listen } from "@tauri-apps/api/event";

const MsgListener = (
  setMsg: React.Dispatch<React.SetStateAction<TMsg | undefined>>
) => {
  useEffect(() => {
    listen<TMsg>("msg", ({ payload }) => {
      setMsg(payload);
    });
  }, []);
}

export default MsgListener;
