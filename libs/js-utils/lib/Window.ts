import { AsyncNmluginUnknown, NmluginUnknown as Nmlugin } from "@nmide/js-utils";

declare global {
  interface Window {
    plugins: Map<string, Nmlugin>;
    async_plugins: Map<string, AsyncNmluginUnknown>
  }
}
