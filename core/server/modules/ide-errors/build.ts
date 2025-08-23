await Bun.build({
  entrypoints: ["./ide-errors.ts"],
  outdir: "../../built_modules/",
  minify: true
})
