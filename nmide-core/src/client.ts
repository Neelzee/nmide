import { invoke } from "@tauri-apps/api/tauri";
import { Msg } from "./bindings/Msg";
import { TSHtml } from "./bindings/TSHtml";

export default function TauriClient<K extends keyof TauriCommands>
  (
    fn: K,
    arg: TauriCommands[K]["args"]
  ): Promise<TauriCommands[K]["ret"]> {
  return invoke<TauriCommands[K]["ret"]>(fn, arg);
}

interface TauriCommands {
  "init_html": {
    args: {},
    ret: TSHtml
  },
  "process_msg": {
    args: {
      msg: Msg
    },
    ret: void,
  }
}
