import React from "react";
import { CodeEditor } from "./code_editor";

type EditorState = {
  openTabs: Tab[];
};

export class EditorTab extends React.Component<any, EditorState> {
  //@ts-ignore
  constructor(props) {
    super(props);

    this.state = { openTabs: [] };
  }

  open_tab(t: Tab) {
    this.setState((prevState) => ({
      ...prevState,
      openTabs: [...prevState.openTabs, t],
    }));
  }

  render() {
    const { openTabs } = this.state;

    return (
      <section className="editor-tabs">
        {openTabs.length === 0 ? (
          <span
            onClick={() =>
              this.open_tab(
                new Tab({ file_name: "New file", file_content: "" })
              )
            }
          >
            Open new
          </span>
        ) : (
          openTabs.map((t) => {
            return t.render();
          })
        )}
      </section>
    );
  }
}

type TabProps = {
  file_name: string;
  file_content: string;
};

type TabState = {
  file_name: string;
  unsaved_changes: boolean;
};

export class Tab extends React.Component<TabProps, TabState> {
  constructor(props: TabProps) {
    super(props);
    this.state = {
      file_name: props.file_name,
      unsaved_changes: false,
    };
  }

  render() {
    return (
      <article id={this.props.file_name}>
        <span>{this.props.file_name}</span>
        <CodeEditor
          file_name={this.props.file_name}
          file_content={this.props.file_content}
        />
      </article>
    );
  }
}
