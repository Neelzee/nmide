/**
 * File Component, used in the explorer tab.
 */

import React from "react";

type Props = {
  name: string;
  path: string;
  icon: "📄";
  onFileChange: (state: FileState) => void;
};

export type FileState = {
  name: string;
  path: string;
  is_open: boolean;
};

export class FileComponent extends React.Component<Props, FileState> {
  constructor(props: Props) {
    super(props);
    this.state = {
      name: props.name,
      path: props.path,
      is_open: false,
    };
  }

  render() {
    const { name, path } = this.state;

    const open_file = () => {
      this.setState((prevState) => {
        return { ...prevState, is_open: true };
      });
    };

    return (
      <div key={path} className={`file ${name}`}>
        <span
          className="file-name"
          onClick={() => {
            open_file();
            this.props.onFileChange(this.state);
          }}
        >{`${this.props.icon} ${name}`}</span>
      </div>
    );
  }
}
