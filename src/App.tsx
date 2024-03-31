import { ToolBar } from "./components/toolbar";
import { Explorer } from "./components/explorer";
import "./styles/main.scss";
import { invoke } from "@tauri-apps/api";
import { trace, info, error, attachConsole } from "tauri-plugin-log-api";

const detach = await attachConsole();

function App() {
  // with LogTarget::Webview enabled this function will print logs to the browser console

  trace("Trace");
  info("Info");
  error("Error");

  // detach the browser console from the log stream
  detach();

  invoke("get_workspace", { path: "/home/nmf/Documents/nmide" })
    .then(res => {
      console.log(res);
    })
    .catch(err => console.error(err));


  return (
    <main>
      <ToolBar />
      <article>
        <Explorer files={["f1"]} />
      </article>
    </main>
  );
}

export default App;
