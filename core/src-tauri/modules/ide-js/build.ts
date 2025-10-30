await Bun.build({
  entrypoints: [
    "./modules/change-tabular.ts",
    "./modules/f-squared.ts",
    "./modules/from-the-rooftop.ts",
    "./modules/post-tabular.ts",
    "./modules/tabular.ts",
  ],
  minify: true,
  outdir: "./build/"
})
