import { AsyncNmluginUnknown, NmluginUnknown as Nmlugin } from "./Nmlugin";
import { THtml } from "./THtml";
import { TMap } from "./TMap";

declare global {
  interface Window {
    pluginAssets: [string, string][];
    cleanup: [string, (() => void)][];
    renderHtml: (html: THtml) => HTMLElement | undefined;
    parseHtml: (html: THtml) => HTMLElement;
    state: TMap;
    root: HTMLElement;
    plugins: Map<string, Nmlugin>;
    async_plugins: Map<string, AsyncNmluginUnknown>;
  }
}
