import React from "react";
import "../styles/menubar.scss";

export class MenuBar extends React.Component {
  render() {
    return (
      <>
        <nav>
          <div>File</div>
          <div>Edit</div>
          <div>View</div>
          <div>Settings</div>
          <div>Help</div>
        </nav>
      </>
    );
  }
}
