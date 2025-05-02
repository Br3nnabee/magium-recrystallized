/*
 * This module provides a Svelte store interface for interacting with the
 * webassembly module. It initializes the WASM runtime, fetches nodes from
 * the story file, caches them, and offers reactive stores for the current
 * story node and navigation.
 */

import { ready, fetchRootNodeFull, fetchNodeFull } from '$lib/wasm';
import { writable, derived, get } from 'svelte/store';

/**
 * Describes a single choice edge from one story node to another.
 */
export interface EdgeInfo {
  /**
   * Label text displayed for this choice.
   */
  label: string;
  /**
   * Numeric index of the destination node when this choice is selected.
   */
  dest: number;
}

/**
 * Represents a story node (scene) in the CYOA engine.
 */
export interface StoryNode {
  /**
   * The narrative content or description for this node.
   */
  content: string;
  /**
   * Array of outgoing edges (choices) from this node.
   */
  edges: EdgeInfo[];
}

/**
 * In-memory cache mapping node indices to their loaded StoryNode data.
 * Uses a Svelte writable store to trigger reactive updates on cache changes.
 */
export const nodeCache = writable<Map<number, StoryNode>>(new Map());

/**
 * Svelte store holding the index of the currently active node in the story.
 */
export const currentIndex = writable<number>(0);

/**
 * Derived store that returns the StoryNode corresponding to currentIndex.
 * Falls back to a placeholder node if the requested index is not yet cached.
 */
export const currentNode = derived(
  [nodeCache, currentIndex],
  ([$cache, $idx]): StoryNode =>
    $cache.get($idx) ?? { content: 'Loading…', edges: [] }
);

/**
 * Initialize the WASM runtime and load the root node (index 0) into cache.
 *
 * Steps:
 * 1. Await the WASM module initialization (ready promise).
 * 2. Fetch the root node data via the Rust-generated API.
 * 3. Store the root node under index 0 in the cache.
 * 4. Set currentIndex to 0, triggering subscribers to display the root.
 *
 * @returns A Promise that resolves once initialization and caching are complete.
 */
export async function initialize(): Promise<void> {
  // Ensure the WASM runtime is loaded
  await ready;

  // Retrieve root node from the WASM engine
  const root = await fetchRootNodeFull();

  // Cache the root node and navigate to it
  nodeCache.update((m) => m.set(0, root));
  currentIndex.set(0);
}

/**
 * Load a specific story node by its index, cache it, and attempt to prefetch its children.
 *
 * - If the node is already cached, this is a no-op.
 * - Otherwise, it fetches the node, updates the cache, and
 *   kicks off background fetches for all child nodes (edges.dest).
 * - Errors during fetch are caught and logged; the cache is updated with an error node.
 *
 * @param idx - Numeric index of the node to load.
 * @returns A Promise that resolves once the node fetch (and prefetch dispatch) is initiated.
 */
export async function loadNode(idx: number): Promise<void> {
  const cache = get(nodeCache);
  // Skip loading if already present
  if (cache.has(idx)) return;

  try {
    // Fetch the node data from WASM
    const node = await fetchNodeFull(idx);
    nodeCache.update(m => m.set(idx, node));

    // Prefetch each child node in parallel, logging any failures
    const childPromises = node.edges.map(({ dest }) =>
      fetchNodeFull(dest)
        .then(child => nodeCache.update(m => m.set(dest, child)))
        .catch(err => {
          console.error(`prefetch failed for node #${dest}:`, err);
        })
    );
  } catch (e) {
    // On error, log and insert an error placeholder in the cache
    console.error(`Error loading node #${idx}:`, e);
    nodeCache.update(m =>
      m.set(idx, { content: `Error: ${e}`, edges: [] })
    );
  }
}

/**
 * Navigate to a different story node by updating currentIndex and triggering a load.
 *
 * @param idx - Numeric index of the target node.
 */
export function goTo(idx: number): void {
  // Update active node index
  currentIndex.set(idx);
  // Begin loading (and caching) the new node asynchronously
  loadNode(idx);
}
