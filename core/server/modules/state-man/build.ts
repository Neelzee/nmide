await Bun.build({
  entrypoints: ["./state-man.ts"],
  outdir: "../../built_modules/",
  minify: true,
});
