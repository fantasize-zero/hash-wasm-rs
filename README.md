# hash-wasm-rs

A WebAssembly library for computing hashes, built with Rust.

# Supported algorithms

| Name  
| ----------------------------------------------  
| BLAKE3  
| MD5  
| SHA: SHA-224, SHA-256, SHA-384, SHA-512
| SHA-3: SHA3-224, SHA3-256, SHA3-384, SHA3-512

# Installation

```bash
pnpm add hash-wasm-rs
```

# Cargo Example Installation

```bash
cargo install hash-wasm-rs
```

# HTML Example usage

[Example large file hash calculation](./index.html)

# Vite Example usage

```bash
pnpm add vite-plugin-wasm
```

### vite.config.ts

```javascript
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig(({ mode }) => ({
  plugins: [wasm()],
}));
```

### app.vue

```javascript
onMounted(async () => {
  try {
    const $hashWasmRs = await import("hash-wasm-rs");
    const result = await $hashWasmRs.md5("Hello, world!");
    console.log(result.hex);
    result.free();
  } catch (error) {
    console.error("WASM error:", error);
  }
});
```

# Development

```bash
# Build package WebAssembly library
wasm-pack build --release
# Build HTML WebAssembly library
wasm-pack build --release --target web --out-dir pkg-web
# Build WebAssembly library with SIMD support
RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --release --target web

# Serve the demo page
python3 -m http.server
```

## LICENSE

[MIT](./LICENSE)
