await Bun.build({
  entrypoints: ["./ide_save.ts"],
  outdir: "../../built_modules/",
  minify: true,
});
