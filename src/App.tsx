import ToolBar from "./components/toolbar";
import Explorer from "./components/explorer";
import "./styles/main.scss";
import { invoke } from "@tauri-apps/api";
import { attachConsole } from "tauri-plugin-log-api";
import ErrorPane from "./components/errorPane";
import { createEffect, createSignal } from "solid-js";
import { NmideReport, NmideError, FolderOrFile, Folder } from "./types";
import { split_with_err } from "./funcs";
import { produce } from "solid-js/store";

const detach = await attachConsole();
// detach the browser console from the log stream
detach();

function App() {
  const [errors, setErrors] = createSignal<NmideReport[]>([]);
  const [folders, setFolders] = createSignal<Folder>({ name: "", path: "", content: [] });
  const [root, setRoot] = createSignal("");

  createEffect(() => {
    if (root() !== "") {
      invoke<NmideError<FolderOrFile>>("get_workspace", { path: root() })
        .then(res => {
          const [val, rep] = split_with_err<FolderOrFile>(res);
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
          setErrors(produce(arr => arr.push(err)));
        });
    }
  });
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
