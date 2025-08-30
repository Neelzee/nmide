await Bun.build({
  entrypoints: ["./server.ts"],
  outdir: "../src-tauri/static",
  minify: true,
})

await Bun.build({
  entrypoints: ["./modules.ts"],
  outdir: "../src-tauri/static",
  minify: true,
})
