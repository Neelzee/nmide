import { ToolBar } from "./components/toolbar";
import { Explorer } from "./components/explorer";
import "./styles/main.scss";
import { invoke } from "@tauri-apps/api";
import { trace, info, error, attachConsole } from "tauri-plugin-log-api";
import { ErrorPane } from "./components/errorPane";
import { createSignal } from "solid-js";
import { NmideReport, NmideError, FolderOrFile, Folder } from "./types";
import { split_with_err } from "./funcs";
import { produce } from "solid-js/store";

const detach = await attachConsole();

function App() {
  const [errors, setErrors] = createSignal<NmideReport[]>([]);
  const [folders, setFolders] = createSignal<Folder>({ name: "", path: "", content: [] });
  const [root, setRoot] = createSignal("");

  // with LogTarget::Webview enabled this function will print logs to the browser console

  trace("Trace");
  info("Info");
  error("Error");

  // detach the browser console from the log stream
  detach();

  if (root() !== "") {
    invoke("get_workspace", { path: root() })
      .then(res => {
        const response = res as NmideError<FolderOrFile>;
        const [val, rep] = split_with_err<FolderOrFile>(response);
        if (rep !== null) {
          setErrors(produce(arr => {
            arr.push(rep);
          }));
        }
        if ("content" in val) {
          setFolders(val);
        } else {
          // Its a file
          setFolders({
            name: "nmide",
            path: "nmide",
            content: [val]
          });
        }
      })
      .catch(err => {
        console.error(err);
        setErrors(produce(arr => arr.push(err)))
      });
  }

  return (
    <main>
      <ToolBar setRoot={setRoot} />
      <article>
        <Explorer files={folders} />
        <ErrorPane errors={errors} />
      </article>
    </main>
  );
}

export default App;
