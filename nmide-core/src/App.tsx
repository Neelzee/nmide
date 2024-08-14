import { useEffect, useState } from "react";
import "./App.css";
import { Html } from "./bindings/Html";
import RenderHtml from "./components/Html";
import TauriClient from "./client";

function App() {
  const [html, setHtml] = useState<Html>("None");

  useEffect(() => {
    TauriClient("init_html", {}).then(html => {
      console.log(html);
      setHtml(html);
    }).catch((err) => {
      console.error(err);
    }).finally(() => { })
  }, []);

  return (
    <RenderHtml html={html} />
  );
}

export default App;
