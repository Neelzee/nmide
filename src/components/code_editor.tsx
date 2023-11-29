import React from "react";

type CEProps = {
  file_name: string;
  file_content: string;
};

type CEState = {
  file_name: string;
  file_content: string;
  unsaved_changes: boolean;
};

export class CodeEditor extends React.Component<CEProps, CEState> {
  constructor(props: CEProps) {
    super(props);
    this.state = {
      file_name: props.file_name,
      file_content: props.file_content,
      unsaved_changes: false,
    };
  }

  render() {
    return (
      <section className="code-editor">
        <textarea
          onChange={() => {
            this.setState((prevState) => ({
              ...prevState,
              unsaved_changes: true,
            }));
          }}
        />
      </section>
    );
  }
}
