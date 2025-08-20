await Bun.build({
  entrypoints: ["./server.ts"],
  outdir: "../../src-tauri/static",
  minify: false,
})
