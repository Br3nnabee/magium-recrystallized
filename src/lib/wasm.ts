import init, { CyoaClient } from '../pkg/wasm_module.js';
import type { CyoaClient as ClientType } from '../pkg/wasm_module.js';

const STORY_PATH = 'magium.story';

const wasmUrl = new URL('../pkg/wasm_module_bg.wasm', import.meta.url);
export const ready = init({ url: wasmUrl.href });

type ClientMap = Record<string, ClientType>;
const clients: ClientMap = {};

async function getClient(): Promise<ClientType> {
  await ready;
  if (!(STORY_PATH in clients)) {
    clients[STORY_PATH] = new CyoaClient(STORY_PATH);
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

export async function fetchNodePassage(nodeIdx: number): Promise<string> {
  const client = await getClient();

  const contentHex: string = await client.get_node_content(nodeIdx);
  console.log('[CYOA] node→content chunk ID:', contentHex);

  const ids = await fetchChunkIds();
  const contentIdx = ids.findIndex(id => id === contentHex);
  if (contentIdx < 0) {
    throw new Error(`Content chunk ${contentHex} not found in index`);
  }

  const txt: string = await client.get_content(contentIdx);
  console.log(`[CYOA] fetchNodePassage [nodeIdx=${nodeIdx}] → chunkIdx=${contentIdx}`, txt);
  return txt;
}

export async function fetchEdges(nodeIdx: number): Promise<string[]> {
  const client = await getClient();
  const jsArr = await client.get_edges(nodeIdx);
  const edges = Array.from(jsArr.values()) as string[];
  console.log(`[CYOA] fetchEdges [${nodeIdx}] →`, edges);
  return edges;
}

export async function fetchEdgeLabel(edgeIdx: number): Promise<string> {
  const client = await getClient();
  const label: string = await client.get_edge_label(edgeIdx);
  console.log(`[CYOA] fetchEdgeLabel [${edgeIdx}] →`, label);
  return label;
}

export async function fetchEdgeDestination(edgeIdx: number): Promise<string> {
  const client = await getClient();
  const destHex: string = await client.get_edge_destination(edgeIdx);
  console.log(`[CYOA] fetchEdgeDestination [${edgeIdx}] →`, destHex);
  return destHex;
}

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
