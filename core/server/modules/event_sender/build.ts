await Bun.build({
  entrypoints: ["./event_sender.ts"],
  outdir: "../../built_modules/",
  minify: true,
}).catch(() => process.exit(1));
