import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [events, setEvents] = useState<string[]>([]);

  useEffect(() => {
    listen("nils", (e) => {
      setEvents(prev => {
        prev.push(e.event);
        return prev;
      })
    }).then(un => {
      return () => un();
    }).catch((err) => {
      console.error(err);
      return () => { };
    })
  }, []);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("greet", {})
  }

  async function test() {
    await invoke("test", {})
  }

  return (
    <div>
      <p>Hello, World!</p>
      <button onClick={greet} />
      <p>{events.map((e) => { return <>{e}</> })}</p>
      <button onClick={test} />
    </div>
  );
}

export default App;
