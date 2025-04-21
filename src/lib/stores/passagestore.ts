import { writable, type Writable } from 'svelte/store';
import { loadContent as _load } from '$lib/wasm';

export const content: Writable<string[]> = writable([]);

export async function loadContent() {
  const arr = await _load();
  content.set(arr);
}
