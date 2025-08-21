import { getCurrentWebview } from "@tauri-apps/api/webview";

const webview = getCurrentWebview();

webview.listen<{ field: string, value: any }>(
  "setDocument",
  ({ payload: { field, value } }) => {
    window[field] = value;
  }
);
