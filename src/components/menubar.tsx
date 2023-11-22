import React from "react";
import "../styles/menubar.scss";

export class MenuBar extends React.Component {
  render() {
    return (
      <>
        <nav>
          <div className="logo">NM</div>
          <div className="btn">File</div>
          <div className="btn">Edit</div>
          <div className="btn">View</div>
          <div className="btn">Settings</div>
          <div className="btn">Help</div>
        </nav>
      </>
    );
  }
}
