await Bun.build({
  entrypoints: ["./server.ts"],
  outdir: "./static",
  minify: true,
})

await Bun.build({
  entrypoints: ["./modules.ts"],
  outdir: "./static",
  minify: true,
})
