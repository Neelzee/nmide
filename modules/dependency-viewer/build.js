await Bun.build({
  entryPoints: ['./index.ts'],
  outdir: "./build/",
  minify: true,
}).catch(() => process.exit(1));

