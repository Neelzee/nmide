"use client"

import { useEffect } from "react";
import Nmlugin from "./Nmlugin";
import { invoke } from "@tauri-apps/api/core";
import "./Window";

const InstallPlugins = (
  setInstalled: React.Dispatch<React.SetStateAction<boolean>>,
  setPlugins: React.Dispatch<React.SetStateAction<Nmlugin[]>>,
) => {
  useEffect(() => {
    if (window === undefined) return;
    invoke<void>("install")
      .then(_ => {
        setPlugins(window.plugins);
        setInstalled(true);
      })
      .catch(err => console.error("Failed installing pugins: ", err));
  }, []);
};

export default InstallPlugins;
