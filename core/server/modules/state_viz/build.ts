await Bun.build({
  entrypoints: ["./state_viz.ts"],
  outdir: "../../built_modules/",
  minify: true,
})
