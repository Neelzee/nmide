import { useEffect, useState } from "react";
import "./App.css";
import { Html } from "./bindings/Html";
import RenderHtml from "./components/Html";
import TauriClient from "./client";
import { listen } from '@tauri-apps/api/event'

function App() {
  const [html, setHtml] = useState<Html>("None");

  useEffect(() => {
    TauriClient("init_html", {}).then(html => {
      setHtml(html);
    }).catch((err) => {
      console.error(err);
    }).finally(() => { })
  }, []);

  useEffect(() => {
    listen("refresh_html", (_) => {
      TauriClient("init_html", {}).then(html => {
        setHtml(html);
      }).catch((err) => {
        console.error(err);
      }).finally(() => { })

    }).then(un => un)
      .catch(err => console.error(err));
  }, []);

  return (
    <RenderHtml html={html} />
  );
}

export default App;
