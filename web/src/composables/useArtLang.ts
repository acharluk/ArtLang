import { ref } from "vue";

const wasmReady = ref(false);
const wasmError = ref(null);
const artlangVersion = ref(null);

let wasmModule: any = null;

async function loadWasm() {
  if (wasmModule) return;

  try {
    //@ts-ignore
    const wasm = await import("@pkg/artlang_wasm.js");
    await wasm.default();
    wasmModule = wasm;
    wasmReady.value = true;

    try {
      artlangVersion.value = wasm.version();
    } catch (_) {}
  } catch (err: any) {
    console.error("Failed to load ArtLang WASM:", err);
    wasmError.value = err.message || "Failed to load WASM module";
  }
}

export function useArtLang() {
  function run(source: string) {
    if (!wasmModule) {
      return {
        success: false,
        output: "",
        error: "WASM module not loaded",
        elapsed: 0,
      };
    }

    const t0 = performance.now();
    let result;
    try {
      result = wasmModule.run_program(source);
    } catch (err: any) {
      result = {
        success: false,
        output: "",
        error: "Internal error: " + (err.message || String(err)),
      };
    }
    const t1 = performance.now();

    return {
      ...result,
      elapsed: t1 - t0,
    };
  }

  return {
    wasmReady,
    wasmError,
    artlangVersion,
    loadWasm,
    run,
  };
}
