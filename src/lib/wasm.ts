// src/lib/wasm.ts
// 1. Grab both the default init-binding and your function
import init, { get_content } from '../pkg/wasm_module.js';

/// 2. Point at the .wasm file (make sure this path ends up being served)
const wasmUrl = new URL('../pkg/wasm_module_bg.wasm', import.meta.url);

/// 3. Initialize as soon as possible
export const ready = init(wasmUrl);

export async function loadContent(): Promise<number[]> {
  // 4. Wait for the module to be fully wired up
  await ready;

  // 5. Now it's safe to call any wasm‑backed function
  const wasmArray = get_content();
  return Array.from(wasmArray);
}

