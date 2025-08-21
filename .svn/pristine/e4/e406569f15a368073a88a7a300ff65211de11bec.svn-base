import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

// TODO: Add docs
export const htmlInstaller = async (url: string): Promise<string | undefined> => {
  if (!url.endsWith(".wb.html")) return url;
  let name = url.split("%2F").pop()?.split(".")[0];
  name = name === undefined ? url : name
  const wb = new WebviewWindow(`nmide-${name}`, { url });
  wb.once("tauri://webview-created", () => {
    wb.setTitle(name)
      .catch(
        err => window.__nmideConfig__.log.error(
          "Failed setting title for webview: ", name, err
        )
      );
  }).catch(
    err => window.__nmideConfig__.log.error("tauri://webview-created", err)
  );
  wb.once("tauri://error", err => {
    window.__nmideConfig__.log.error(
      `Error from webview: ${name}: `, err
    );
  }).catch(
    err =>
      window.__nmideConfig__.log.error(
        "Error on tauri://Error listening on webview: ", name, err
      )
  );
  return;
};
