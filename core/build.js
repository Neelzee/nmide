import { build } from "esbuild";

build({
  entryPoints: ["./index.js", "./app/setup.ts"],
  bundle: true,
  outdir: "./build",
  minify: false,
  platform: "neutral",
  external: ["timers/promises"],
}).catch(err => {
  console.error(err);
  process.exit(1);
});

