import init, { CyoaGame } from '../pkg/wasm_module.js';
import type { CyoaGame as ClientType } from '../pkg/wasm_module.js';

// Define the story file here so components don't pass it around
const STORY_PATH = 'magium.story';

const wasmUrl = new URL('../pkg/wasm_module_bg.wasm', import.meta.url);
export const ready = init({ url: wasmUrl.href });

type ClientMap = Record<string, ClientType>;
const clients: ClientMap = {};

/**
 * Instantiate (or return) the single CyoaClient for our STORY_PATH.
 */
async function getClient(): Promise<ClientType> {
  await ready;
  if (!(STORY_PATH in clients)) {
    clients[STORY_PATH] = new CyoaGame(STORY_PATH);
    console.log('[CYOA] instantiated client for', STORY_PATH);
  }
  return clients[STORY_PATH];
}

/**
 * Fetch the complete list of chunk IDs for the story.
 */
export async function fetchChunkIds(): Promise<string[]> {
  const client = await getClient();
  const ids = Array.from(client.chunk_ids().values()) as string[];
  console.log('[CYOA] fetchChunkIds →', ids.length, 'chunks');
  return ids;
}

/**
 * Fetch the actual text for a node by first mapping node → content chunk.
 */
export async function fetchNodePassage(nodeIdx: number): Promise<string> {
  const client = await getClient();

  // 1) ask WASM which chunk holds this node's content
  const contentHex: string = await client.get_node_content(nodeIdx);
  console.log('[CYOA] node→content chunk ID:', contentHex);

  // 2) locate that hex in the full chunk list
  const ids = await fetchChunkIds();
  const contentIdx = ids.findIndex(id => id === contentHex);
  if (contentIdx < 0) {
    throw new Error(`Content chunk ${contentHex} not found in index`);
  }

  // 3) fetch the text from the correct chunk index
  const txt: string = await client.get_content(contentIdx);
  console.log(`[CYOA] fetchNodePassage [nodeIdx=${nodeIdx}] → chunkIdx=${contentIdx}`, txt);
  return txt;
}

/**
 * Alias for backward compatibility: fetchContent(nodeIdx) → fetchNodePassage(nodeIdx)
 */
export const fetchContent = fetchNodePassage;

/**
 * Fetch the outgoing edge IDs (hex strings) for a node.
 */
export async function fetchEdges(nodeIdx: number): Promise<string[]> {
  const client = await getClient();
  const jsArr = await client.get_edges(nodeIdx);
  const edges = Array.from(jsArr.values()) as string[];
  console.log(`[CYOA] fetchEdges [${nodeIdx}] →`, edges);
  return edges;
}

/**
 * Fetch the human-readable label for an edge chunk index.
 */
export async function fetchEdgeLabel(edgeIdx: number): Promise<string> {
  const client = await getClient();
  const label: string = await client.get_edge_label(edgeIdx);
  console.log(`[CYOA] fetchEdgeLabel [${edgeIdx}] →`, label);
  return label;
}

/**
 * Fetch the destination chunk hex ID for an edge chunk index.
 */
export async function fetchEdgeDestination(edgeIdx: number): Promise<string> {
  const client = await getClient();
  const destHex: string = await client.get_edge_destination(edgeIdx);
  console.log(`[CYOA] fetchEdgeDestination [${edgeIdx}] →`, destHex);
  return destHex;
}

/**
 * Fetch and resolve the root node index for the story.
 */
export async function fetchRootIndex(): Promise<number> {
  const client = await getClient();
  const rootHex: string = await client.get_root_node();
  console.log('[CYOA] root chunk ID:', rootHex);

  const ids = await fetchChunkIds();
  const idx = ids.findIndex(id => id === rootHex);
  if (idx < 0) throw new Error(`Root chunk ${rootHex} not in index`);

  console.log('[CYOA] root index =', idx);
  return idx;
}
