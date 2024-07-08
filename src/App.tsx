import ToolBar from "@components/toolbar";
import Explorer from "@components/explorer";
import { ExplorerProps } from "@components/explorer";
import "@styles/main.scss";
import { invoke } from "@tauri-apps/api";
import ErrorPane from "@components/errorPane";
import { createEffect, createSignal, JSX, Accessor, Setter } from "solid-js";
import { split_with_err } from "./lib/funcs";
import { produce } from "solid-js/store";
import { Dynamic } from "solid-js/web";
import { NmideReport } from "./lib/models/NmideReport";
import { Folder } from "./lib/models/Folder";
import { FolderOrFile } from "./lib/models/FolderOrFile";
import { NmideError } from "./lib/models/NmideError";


function App() {
  const [errors, setErrors] = createSignal<NmideReport[]>([]);
  const [folders, setFolders] = createSignal<Folder>({
    name: "",
    path: "",
    content: [],
    symbol: ""
  });
  const [root, setRoot] = createSignal("");
  const [pages, setPages] = createSignal<((props: any) => JSX.Element)[]>([]);
  const [content, setContent] = createSignal<string[]>([]);
  const [loading, setLoading] = createSignal(false);

  const explorer = (props: ExplorerProps) => Explorer(props);
  const errorPane = (props: { errors: Accessor<NmideReport[]> }) => ErrorPane(props);

  setPages([explorer, errorPane]);

  createEffect(() => {
    if (root() !== "") {
      setLoading(true);
      invoke<NmideError>("get_workspace", { path: root() })
        .then(res => {
          const [val, rep] = split_with_err<FolderOrFile>(res);
          if (rep !== undefined) {
            setErrors(produce(arr => {
              arr.push(rep);
            }));
          }
          if ("content" in val) {
            setFolders(val);
          } else {
            // Its a file
            setFolders({
              name: val.name,
              path: val.path,
              content: [val],
              symbol: val.symbol,
            });
          }
        })
        .catch(err => {
          setErrors(produce(arr => arr.push(err)));
        })
        .finally(() => setLoading(false));
    }
  });



  return (
    <main>
      <ToolBar setRoot={setRoot} />
      <article>
        <Dynamic
          component={pages()[0]}
          files={folders}
          errors={errors}
          content={content}
          curPage={setPages}
          loading={loading}
        />
      </article>
    </main>
  );
}

export default App;
