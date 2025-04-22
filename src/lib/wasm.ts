import init, { CyoaClient } from '../pkg/wasm_module.js';
import type { CyoaClient as ClientType } from '../pkg/wasm_module.js';

// Initialize the WASM module only once
const wasmUrl = new URL('../pkg/wasm_module_bg.wasm', import.meta.url);
const ready = init({ url: wasmUrl.href });

type ClientMap = Record<string, ClientType>;
const clients: ClientMap = {};
const chunkIdCache: Record<string, string[]> = {};
const rootCache: Record<string, number> = {};

/**
 * Ensure the client for a given story path is instantiated once.
 */
async function getClient(path: string): Promise<ClientType> {
  await ready;
  if (!(path in clients)) {
    clients[path] = new CyoaClient(path);
    console.log('[CYOA] instantiated client for', path);
  }
  return clients[path];
}

/**
 * Normalize a story path to a URL. Root-relative or absolute URLs are returned as-is;
 * otherwise the path is assumed to live under the static dir at web root.
 */
function canonicalize(path: string): string {
  if (path.startsWith('/') || /^https?:\/\//.test(path)) {
    return path;
  }
  return `/${path}`;
}

/**
 * Load and cache the array of all chunk IDs (hex strings) for a given story.
 */
export async function fetchChunkIds(rawPath: string): Promise<string[]> {
  const path = canonicalize(rawPath);
  if (chunkIdCache[path]) return chunkIdCache[path];

  const client = await getClient(path);
  const ids = Array.from(client.chunk_ids().values()) as string[];
  chunkIdCache[path] = ids;
  console.log('[CYOA] loaded chunk IDs (', ids.length, ') for', path);
  return ids;
}

/**
 * Load and cache the root node index for a given story.
 */
export async function fetchRootIndex(rawPath: string): Promise<number> {
  const path = canonicalize(rawPath);
  if (rootCache[path] !== undefined) return rootCache[path];

  const client = await getClient(path);
  const rootHex = await client.get_root_node();
  const ids = await fetchChunkIds(path);
  const idx = ids.findIndex(id => id === rootHex);
  if (idx < 0) throw new Error(`Root chunk ${rootHex} not found`);
  rootCache[path] = idx;
  console.log('[CYOA] root index =', idx, 'for', path);
  return idx;
}

/**
 * Fetch the text content for a specific chunk index.
 */
export async function fetchContent(rawPath: string, idx: number): Promise<string> {
  const path = canonicalize(rawPath);
  const client = await getClient(path);
  return await client.get_content(idx);
}

/**
 * Fetch the array of outgoing edge IDs (hex strings) for a node.
 */
export async function fetchEdges(rawPath: string, nodeIdx: number): Promise<string[]> {
  const path = canonicalize(rawPath);
  const client = await getClient(path);
  return Array.from((await client.get_edges(nodeIdx)).values()) as string[];
}

/**
 * Fetch the label for an edge by its chunk index in the file.
 */
export async function fetchEdgeLabel(rawPath: string, edgeIdx: number): Promise<string> {
  const path = canonicalize(rawPath);
  const client = await getClient(path);
  return await client.get_edge_label(edgeIdx);
}

/**
 * Fetch the destination chunk ID (hex string) for an edge.
 */
export async function fetchEdgeDestination(rawPath: string, edgeIdx: number): Promise<string> {
  const path = canonicalize(rawPath);
  const client = await getClient(path);
  return await client.get_edge_destination(edgeIdx);
}
