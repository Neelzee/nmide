import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

// (path: string) => Promise<string | undefined>)
export const InstallHtmlPlugin = async (url: string): Promise<string | undefined> => {
  if (!url.endsWith(".wb.html")) return url;
  let name = url.split("%2F").pop()?.split(".")[0];
  name = name === undefined ? url : name
  window.pluginAssets.push([name, url]);
  const wb = new WebviewWindow(`nmide-${name}`, { url });
  wb.once("tauri://webview-created", () => {
    wb.setTitle(name)
      .then(_ => _);
  });
  wb.once("tauri://error", err => {
    console.error(`Error in creating webview: ${name}: `, err);
  });
  return;
};
