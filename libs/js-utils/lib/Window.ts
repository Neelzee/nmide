import { AsyncNmluginUnknown, NmluginUnknown as Nmlugin } from "./Nmlugin";
import { THtml } from "./THtml";
import { TMap } from "./TMap";
import { NmideClient, NmideLogger, Payload } from "./App";

declare global {
  interface Window {
    pluginAssets: [string, string][];
    cleanup: [string, (() => void)][];
    renderHtml: (html: THtml) => HTMLElement | undefined;
    parseHtml: (html: THtml) => HTMLElement | undefined;
    state: TMap;
    root: HTMLElement;
    plugins: Map<string, Nmlugin>;
    async_plugins: Map<string, AsyncNmluginUnknown>;
    client: NmideClient;
    log: NmideLogger;
    listen: <T>(event: string, handler: (x: Payload<T>) => void) => Promise<any>,
    emit: <T>(event: string, payload?: T) => Promise<void>,
    getPluginPaths: Promise<string[]>,
    pluginInstallers: ((path: string) => Promise<string | undefined>)[],
  }
}
