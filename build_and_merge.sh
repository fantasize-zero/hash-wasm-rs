#!/bin/bash

# 清理并构建 Node.js 版本
echo "Building for Node.js..."
wasm-pack build --release --target nodejs --out-dir pkg-node --out-name node

# 构建 Web 版本
echo "Building for Web..."
wasm-pack build --release --target web --out-dir pkg-web --out-name browser

# 复制文件
cp pkg-node/node.js pkg/src/node
cp pkg-node/node_bg.wasm pkg/src/node
cp pkg-node/*.d.ts pkg/src/node
cp pkg-node/README.md pkg/
cp pkg-node/LICENSE pkg/
cp pkg-node/package.json pkg/

cp pkg-web/browser.js pkg/src/browser
cp pkg-web/browser_bg.wasm pkg/src/browser
cp pkg-web/*.d.ts pkg/src/browser

# 更新 package.json
echo "Updating package.json for multi-environment..."
node update_package_json.js

# 下载node包
echo "Install Package"
cd pkg
npm install rollup rollup-plugin-dts @rollup/plugin-node-resolve rollup-plugin-swc3

# 打包
echo "Building..."
npm run build