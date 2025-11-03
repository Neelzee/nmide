await Bun.build({
  entrypoints: ["./server.ts"],
  outdir: "../build",
  minify: true,
})
