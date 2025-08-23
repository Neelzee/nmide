await Bun.build({
  entrypoints: ["./ide-editor.ts"],
  outdir: "../../built_modules/",
  minify: true
})
