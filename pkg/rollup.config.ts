import { defineConfig } from "rollup";
import { dts } from "rollup-plugin-dts";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import { swc } from "rollup-plugin-swc3";

const bundleName = "HashWasmRs";

export default defineConfig([
  // 浏览器 esm 产物
  {
    input: "src/index.ts",
    output: [
      { file: "dist/index.esm.js", format: "esm", exports: "named" },
      { file: "dist/index.cjs.js", format: "cjs", exports: "named" },
    ],
    plugins: [nodeResolve(), swc({ sourceMaps: true })],
  },
  // 浏览器 esm 类型产物
  {
    input: "src/index.ts",
    output: { file: "dist/index.d.ts" },
    plugins: [dts()],
  },
  // 浏览器 iife 产物
  {
    input: "src/iife.ts",
    output: { file: "dist/global.js", format: "iife", name: bundleName },
    plugins: [nodeResolve(), swc({ sourceMaps: true })],
  },
  // 浏览器 iife 类型产物
  {
    input: "src/iife.ts",
    output: { file: "dist/global.d.ts", format: "es" },
    plugins: [dts()],
  },
  // node esm, cjs 产物
  {
    input: "src/node.ts",
    output: [
      { file: "dist/node.mjs", format: "esm", exports: "named" },
      { file: "dist/node.cjs", format: "cjs", exports: "named" },
    ],
    plugins: [nodeResolve(), swc({ sourceMaps: true })],
  },
  // node 类型产物
  {
    input: "src/node.ts",
    output: { file: "dist/node.d.ts" },
    plugins: [dts()],
  },
]);
