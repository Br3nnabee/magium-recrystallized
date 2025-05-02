/*
 * This module initializes and interacts with the webassembly module, providing functions to load and
 * retrieve nodes (scenes) from a story file.
 *
 * It ensures a singleton client per story path and offers convenient methods to fetch the root node
 * or any node by index, returning structured data suitable for a JavaScript/TypeScript client application.
 */

import init, { CyoaGame } from '../pkg/wasm_module.js';
import type { CyoaGame as ClientType } from '../pkg/wasm_module.js';
import { base } from '$app/paths';

/**
 * Relative URL to the compiled WebAssembly binary for the CYOA engine.
 * Constructed at runtime based on the current module location.
 */
const wasmUrl = new URL('../pkg/wasm_module_bg.wasm', import.meta.url);

/**
 * Absolute path (URL) to the story file to load into the WASM engine.
 * Combines SvelteKit's the `base` path alias with the story file name.
 */
export const STORY_PATH = `${base}/magium.story`;

/**
 * Promise that resolves when the WASM module has been initialized.
 * Callers should await this before creating or using the CyoaGame client.
 */
export const ready = init({ url: wasmUrl.href });

/**
 * Low-level representation of a single game edge (choice) returned from WASM.
 * @internal
 */
type EdgeRaw = {
  /** Text label for the choice presented to the player */
  label: string;
  /** Zero-based index of the destination node */
  dest_idx: number;
};

/**
 * Low-level representation of a game node (scene) returned from WASM.
 * @internal
 */
type NodeRaw = {
  /** Scene text or content to display */
  content: string;
  /** Array of outgoing edges (choices) from this node */
  edges: EdgeRaw[];
};

/**
 * Map from story file path to its singleton CyoaGame client instance.
 * Ensures we only ever instantiate one WASM client per story.
 */
type ClientMap = Record<string, ClientType>;
const clients: ClientMap = {};

/**
 * Get (or create) the singleton WASM client for the configured story.
 * Ensures the WASM module is fully initialized before instantiation.
 *
 * @returns Promise resolving to the CyoaGame client instance.
 */
async function getClient(): Promise<ClientType> {
  // Wait until the WASM module is ready
  await ready;

  // Lazy-instantiation: create the client if it doesn't exist
  if (!(STORY_PATH in clients)) {
    clients[STORY_PATH] = new CyoaGame(STORY_PATH);
  }

  return clients[STORY_PATH];
}

/**
 * Public-facing format for edges returned to the application.
 */
export type Edge = {
  /** Text label for the choice */
  label: string;
  /** Destination node index for this choice */
  dest: number;
};

/**
 * Fetch the "root" node (index 0) from the story in a single call.
 *
 * Internally calls the Rust->WASM helper `load_root_node_full`,
 * then transforms the raw data into a more ergonomic shape.
 *
 * @example
 * ```ts
 * const root = await fetchRootNodeFull();
 * console.log(root.content);
 * root.edges.forEach(edge => console.log(edge.label, edge.dest));
 * ```
 *
 * @returns Promise resolving to an object with:
 * - `content`: the root scene text
 * - `edges`: array of choices with labels and destination indices
 *
 * @throws if the WASM module is not ready or fails to load the node.
 */
export async function fetchRootNodeFull(): Promise<{
  content: string;
  edges: Edge[];
}> {
  const client = await getClient();
  // Load the root node (index 0) via the WASM API
  const jsNode = (await client.load_root_node_full()) as NodeRaw;

  // Transform raw edges into public-facing Edge objects
  const edges: Edge[] = jsNode.edges.map(({ label, dest_idx }) => ({
    label,
    dest: dest_idx,
  }));

  return {
    content: jsNode.content,
    edges,
  };
}

/**
 * Fetch any node by its zero-based index in the story.
 *
 * Internally calls the Rust->WASM helper `load_node_full`,
 * then transforms the raw data into a more ergonomic shape.
 *
 * @param nodeIdx - Zero-based index of the node to load
 *
 * @example
 * ```ts
 * const scene = await fetchNodeFull(5);
 * console.log(scene.content);
 * ```
 *
 * @returns Promise resolving to an object with:
 * - `content`: the scene text
 * - `edges`: array of edges (choices)
 *
 * @throws if `nodeIdx` is out of range or the WASM call fails.
 */
export async function fetchNodeFull(
  nodeIdx: number
): Promise<{
  content: string;
  edges: Edge[];
}> {
  const client = await getClient();
  const jsNode = (await client.load_node_full(nodeIdx)) as NodeRaw;

  // Map fields from WASM format to our public API
  const edges: Edge[] = jsNode.edges.map(({ label, dest_idx }) => ({
    label,
    dest: dest_idx,
  }));

  return {
    content: jsNode.content,
    edges,
  };
}
