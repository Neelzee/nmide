import * as fs from "fs";

const indexPath = "../build/index.html";
try {
  if (fs.existsSync(indexPath)) {
    fs.copyFileSync(indexPath, "../static/index.html");
  }
} catch (err) {
  console.error("Error copying: ", indexPath, err);
}
const faviconPath = "./favicon.ico";
try {
  if (fs.existsSync(faviconPath)) {
    fs.copyFileSync(faviconPath, "../static/favicon.ico");
  }
} catch (err) {
  console.error("Error copying: ", indexPath, err);
}

await Bun.build({
  entrypoints: ["../static/modules.js"],
  outdir: "../static/",
  minify: true,
  splitting: true,
});

console.log("Finished");
