import { useEffect } from "react";
import { TMsg } from "./bindings/TMsg";
import { listen } from "@tauri-apps/api/event";

const MsgListener = (setMsg: React.Dispatch<React.SetStateAction<TMsg>>) => {
  useEffect(() => {
    listen("msg", ({ payload }) => {
      console.log(payload);
    });
  }, []);
}
