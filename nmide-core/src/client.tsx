import { Html } from "./bindings/Html"
import { invoke } from "@tauri-apps/api/tauri";

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
    ret: Html
  }
}
