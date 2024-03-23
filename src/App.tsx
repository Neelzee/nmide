import { ToolBar } from "./components/toolbar";
import { Explorer } from "./components/explorer";
import "./styles/main.scss";
import { invoke } from "@tauri-apps/api";

function App() {

  invoke("get_workspace", { path: "C:\\Users\\nilsi\\Documents\\nmide" })
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
