import { Setter } from "solid-js";
import "../styles/toolbar.scss";
import { open } from '@tauri-apps/api/dialog';

export function ToolBar(props: { setRoot: Setter<string> }) {

  const openFolder = () => {
    open({
      directory: true
    }).then(res => {
      const root = res as string;
      props.setRoot(root);
    })
      .catch(err => console.error(err));
  }


  return (
    <section id="toolbar">
      <span class="toolbar-elem logo">
        Logo
      </span>
      <span class="toolbar-elem" onClick={openFolder}>
        File
      </span>
      <span class="toolbar-elem">
        Edit
      </span>
      <span class="toolbar-elem">
        Selection
      </span>
      <span class="toolbar-elem">
        View
      </span>
      <span class="toolbar-elem">
        Go
      </span>
      <span class="toolbar-elem">
        Run
      </span>
      <span class="toolbar-elem">
        Terminal
      </span>
      <span class="toolbar-elem">
        Help
      </span>
    </section>
  );
}
