import { defineConfig } from "rollup";
import { dts } from "rollup-plugin-dts";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import { swc } from "rollup-plugin-swc3";
import commonjs from "@rollup/plugin-commonjs";
import copy from "rollup-plugin-copy";

const bundleName = "HashWasmRs";

export default defineConfig([
  // 浏览器 esm 产物
  {
    input: "src/index.ts",
    output: [
      { file: "dist/index.esm.js", format: "esm", exports: "named" },
      { file: "dist/index.cjs.js", format: "cjs", exports: "named" },
    ],
    plugins: [
      nodeResolve(),
      commonjs(),
      swc({ sourceMaps: true }),
      copy({
        targets: [
          { src: "../pkg-web/browser_bg.wasm", dest: "dist" },
          { src: "../pkg-web/*.d.ts", dest: "dist/types" },
        ],
      }),
    ],
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
    plugins: [
      nodeResolve(),
      commonjs(),
      swc({ sourceMaps: true }),
      copy({
        targets: [{ src: "../pkg-web/browser_bg.wasm", dest: "dist" }],
      }),
    ],
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
    plugins: [
      nodeResolve(),
      commonjs(),
      swc({ sourceMaps: true }),
      copy({
        targets: [
          { src: "../pkg-node/node_bg.wasm", dest: "dist" },
          { src: "../pkg-node/*.d.ts", dest: "dist/types" },
        ],
      }),
    ],
  },
  // node 类型产物
  {
    input: "src/node.ts",
    output: { file: "dist/node.d.ts" },
    plugins: [dts()],
  },
]);
