await Bun.build({
  entrypoints: ["./index.ts"],
  outdir: "./build/",
  minify: true,
})
