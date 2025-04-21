import { writable, type Writable } from 'svelte/store';
import init, { get_content } from 'wasm_module';

export const content: Writable<string[]> = writable<string[]>([]);;

// Kick off the WASM loader once:
const ready = init();

// Expose a loader function that components can call:
export async function loadContent() {
  await ready;
  // get_content returns a js_sys::Array – it behaves like a JS Array of strings
  const wasmArray = get_content();
  // Convert to a real JS array (so it serializes, maps, etc.)
  const jsArr = Array.from(wasmArray);
  content.set(jsArr);
}
