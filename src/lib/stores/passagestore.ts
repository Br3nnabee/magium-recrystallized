import { ready, fetchRootNodeFull, fetchNodeFull } from '$lib/wasm';
import { writable, derived, get } from 'svelte/store';

export interface EdgeInfo {
  label: string;
  dest: number;
}

export interface StoryNode {
  content: string;
  edges: EdgeInfo[];
}

export const nodeCache = writable<Map<number, StoryNode>>(new Map());
export const currentIndex = writable<number>(0);

export const currentNode = derived(
  [nodeCache, currentIndex],
  ([$cache, $idx]): StoryNode =>
    $cache.get($idx) ?? { content: 'Loading…', edges: [] }
);

/**
 * Initialize the WASM runtime and load the root node.
 */
export async function initialize(): Promise<void> {
  await ready;

  const root = await fetchRootNodeFull();
  nodeCache.update((m) => m.set(0, root));
  currentIndex.set(0);
}

/**
 * Fetch (and cache) an arbitrary node by its numeric index.
 */

export async function loadNode(idx: number): Promise<void> {
  const cache = get(nodeCache);
  if (cache.has(idx)) return;

  try {
    const node = await fetchNodeFull(idx);
    nodeCache.update(m => m.set(idx, node));

    const childPromises = node.edges.map(({ dest }) =>
      fetchNodeFull(dest)
        .then(child => nodeCache.update(m => m.set(dest, child)))
        .catch(err => {
          console.error(`prefetch failed for node #${dest}:`, err);
        })
    );
  } catch (e) {
    console.error(`Error loading node #${idx}:`, e);
    nodeCache.update(m =>
      m.set(idx, { content: `Error: ${e}`, edges: [] })
    );
  }
}

/**
 * Navigate to a different node by its numeric index.
 */
export function goTo(idx: number): void {
  currentIndex.set(idx);
  loadNode(idx);
}
