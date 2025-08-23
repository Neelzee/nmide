await Bun.build({
  entrypoints: ["./dependency-viewer.ts"],
  outdir: "../../built_modules/",
  minify: true,
}).catch(() => process.exit(1));

