import { EditorWindow } from "./components/editor_window";
import { Explorer } from "./components/explorer";
import { MenuBar } from "./components/menubar";
import "./styles/main.scss";

function App() {
  return (
    <>
      <MenuBar />
      <main>
        <Explorer />
        <EditorWindow />
      </main>
    </>
  );
}

export default App;
