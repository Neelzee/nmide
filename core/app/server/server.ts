import { run } from "../lib/run";
import app from "./lib/app";
import config from "./lib/config";

document.addEventListener("DOMContentLoaded", () => {
  run(app, config);
});