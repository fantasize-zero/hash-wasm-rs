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
const result = new HasherWrapper(text, HashType.MD5).result();
console.log(result.hex);

// Compute hash from file data
const file = new File(["Hello, world!"], "hello.txt");
const result = new HasherWrapper(file, HashType.MD5).result();
console.log(result.hex);
```

## Development

```bash
# Build WebAssembly library
wasm-pack build --target web --release

# Serve the demo page
python3 -m http.server
```

## LICENSE

[MIT](./LICENSE)
