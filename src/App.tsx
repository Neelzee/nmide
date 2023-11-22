import { EditorWindow } from "./components/editor_window";
import { MenuBar } from "./components/menubar";
import "./styles/main.scss";

function App() {
  return (
    <>
      <main>
        <MenuBar />
        <EditorWindow />
      </main>
    </>
  );
}

export default App;
