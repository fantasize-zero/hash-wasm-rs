importScripts("https://unpkg.com/comlink/dist/umd/comlink.js");

// 初始化 WASM 模块
async function initWasm() {
  if (!self.WasmModule) {
    const wasmModule = await import("./pkg-web/hash_wasm_rs.js");
    await wasmModule.default();
    self.WasmModule = wasmModule;
  }
}

self.onmessage = async (e) => {
  try {
    await initWasm();

    const data = e.data;
    if (data.type === "process") {
      const chunkData = new Uint8Array(data.data);
      const hasher = new self.WasmModule.HasherWrapper(
        data.hashType,
        chunkData
      );
      const result = await hasher.result();
      self.postMessage({
        type: "chunk_result",
        hash: result.hex,
        index: data.index,
      });
    }
  } catch (error) {
    self.postMessage({
      type: "error",
      message: error.message,
    });
  }
};
