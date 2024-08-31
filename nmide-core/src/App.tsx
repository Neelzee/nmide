import { useEffect, useState } from "react";
import RenderHtml from "./components/Html";
import TauriClient from "./client";
import { listen } from '@tauri-apps/api/event'
import { EmitMsgPayload } from "./bindings/EmitMsgPayload";
import { TSHtml } from "./bindings/TSHtml";

function App() {
  const [html, setHtml] = useState<TSHtml>(
    {
      kind: "Text",
      kids: [],
      text: null,
      attrs: []
    }
  );

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

  useEffect(() => {
    listen<EmitMsgPayload>("emit_msg", ({ payload }) => {
      TauriClient("process_msg", { msg: payload })
        .catch((err) => {
          console.error(err);
        }).finally(() => { })
    }).then(un => un)
      .catch(err => console.error(err));
  }, []);

  return (
    <RenderHtml kind={html.kind} kids={html.kids} text={html.text} attrs={html.attrs} />
  );
}

export default App;
