# hash-wasm-rs

A WebAssembly library for computing hashes, built with Rust.

## Features

- Supports multiple hash algorithms: SHA-256, SHA-512, SHA3-256, SHA3-512, MD5, BLAKE3.

## Installation

```bash
pnpm add hash-wasm-rs
```

## Usage

```javascript
import initWasm, { HasherWrapper, HashType } from "hash-wasm-rs";

await initWasm();

// Compute hash from text input
const text = "Hello, world!";
const hasher = new HasherWrapper(HashType.MD5, text);
const result = await hasher.result();
console.log(result.hex);

// Compute hash from file data
const file = new File(["Hello, world!"], "hello.txt");
const hasher = new HasherWrapper(HashType.MD5, file);
const result = await hasher.result();
console.log(result.hex);
```

## Example usage

[Example large file hash calculation](./index.html)

## Development

```bash
# Build WebAssembly library
wasm-pack build --release --target nodejs --out-dir pkg-node --out-name node
wasm-pack build --release --target web --out-dir pkg-web --out-name browser
# Build WebAssembly library with SIMD support
RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --release --target web --out-dir pkg-web --out-name browser

# Serve the demo page
python3 -m http.server
```

## LICENSE

[MIT](./LICENSE)
