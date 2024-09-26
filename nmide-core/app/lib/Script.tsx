import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir } from "@tauri-apps/api/path";
import { pipe } from "fp-ts/lib/function";
import { join } from "path";
import { useEffect } from "react"

const PluginScript = (pluginPath: string) => {
  useEffect(() => {
    const script = document.createElement("script");
    appDataDir()
      .then(path => pipe(
        path,
        path => join(path, pluginPath),
        convertFileSrc,
        src => script.src = src,
      ))
      .catch(err => console.error(err));
    document.body.appendChild(script);
    return () => {
      document.body.removeChild(script);
    }
  }, [pluginPath]);
}
