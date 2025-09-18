// update_package_json.js
const fs = require("fs");
const path = require("path");

const pkgPath = path.join(__dirname, "pkg", "package.json");
const pkgData = JSON.parse(fs.readFileSync(pkgPath, "utf8"));

pkgData.files = [
  "dist",
  "README.md",
  "README-zh.md",
  "package.json",
  "LICENSE",
];

pkgData.scripts = {
  dev: "rollup --config rollup.config.ts --configPlugin swc3 --watch",
  build: "pnpm rm:dist && rollup --config rollup.config.ts --configPlugin swc3",
  test: "jest --coverage",
  "rm:dist": "rm -rf ./dist",
};
pkgData.type = "module";
pkgData.types = "./dist/index.d.ts";
pkgData.main = "./dist/index.cjs.js";
pkgData.module = "./dist/index.esm.js";

pkgData.exports = {
  ".": {
    types: "./dist/index.d.ts",
    import: "./dist/index.esm.js",
    require: "./dist/index.cjs.js",
  },
  "./browser": {
    types: "./dist/index.d.ts",
    import: "./dist/index.esm.js",
    require: "./dist/index.cjs.js",
  },
  "./node": {
    types: "./dist/node.d.ts",
    import: "./dist/node.mjs",
    require: "./dist/node.cjs",
  },
  "./global": {
    types: "./dist/global.d.ts",
    default: "./dist/global.js",
  },
};

fs.writeFileSync(pkgPath, JSON.stringify(pkgData, null, 2));
