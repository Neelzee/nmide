await Bun.build({
  entrypoints: ["./build/external/modules.js"],
  outdir: "./build/",
  minify: true,
})
