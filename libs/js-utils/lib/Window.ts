import { AsyncNmluginUnknown, NmluginUnknown as Nmlugin } from "./Nmlugin";
import { THtml } from "./THtml";
import { TMap } from "./TMap";

declare global {
  interface Window {
    cleanup: [string, HTMLElement][];
    renderHtml: (html: THtml) => HTMLElement;
    state: TMap;
    plugins: Map<string, Nmlugin>;
    async_plugins: Map<string, AsyncNmluginUnknown>;
  }
}
