/* @refresh reload */
import { render } from "solid-js/web";

import { attachConsole } from "tauri-plugin-log-api";
import App from "./App";
// with LogTarget::Webview enabled this function will print logs to the browser console

//const detach = await attachConsole();

// detach the browser console from the log stream
//detach();
render(() => <App />, document.getElementById("root") as HTMLElement);

