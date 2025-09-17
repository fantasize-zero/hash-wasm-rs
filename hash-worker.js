importScripts("https://unpkg.com/comlink/dist/umd/comlink.js");

let hashWorker = null;
let algorithm = "";

// 初始化 WASM 模块
async function initWasm() {
  if (!self.WasmModule) {
    const wasmModule = await import("./pkg/hash_wasm_rs.js");
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
      algorithm = data.algorithm;
      if (!hashWorker) {
        // 传入算法类型和分片数据，创建 HashWorker
        hashWorker = new self.WasmModule.HashWorker(algorithm, chunkData);
      }
      const hash = await hashWorker.process_chunk();
      self.postMessage({
        type: "chunk_result",
        hash: hash,
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
