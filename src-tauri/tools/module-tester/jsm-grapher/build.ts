await Bun.build({
  entrypoints: ["./jsm-invoker.ts"],
  minify: true,
  target: "bun",
  outdir: "../build/"
})
