await Bun.build({
  entrypoints: ["./dist/external/modules.js"],
  outdir: "./dist/external/",
  minify: true,
})