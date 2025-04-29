import init, { CyoaGame } from '../pkg/wasm_module.js';
import type { CyoaGame as ClientType } from '../pkg/wasm_module.js';
import { base } from '$app/paths';

export const STORY_PATH = `${base}/magium.story`;
const wasmUrl = new URL('../pkg/wasm_module_bg.wasm', import.meta.url);
export const ready = init({ url: wasmUrl.href });

type EdgeRaw = { label: string; dest_idx: number };
type NodeRaw = { content: string; edges: EdgeRaw[] };

type ClientMap = Record<string, ClientType>;
const clients: ClientMap = {};

/** Get or create the singleton CyoaGame */
async function getClient(): Promise<ClientType> {
  await ready;
  if (!(STORY_PATH in clients)) {
    clients[STORY_PATH] = new CyoaGame(STORY_PATH);
  }
  return clients[STORY_PATH];
}

/**
 * Fetch the “root” node (content + edges) in one call.
 */
export async function fetchRootNodeFull(): Promise<{
  content: string;
  edges: { label: string; dest: number }[];
}> {
  const client = await getClient();
  // call the Rust->WASM helper
  const jsNode = (await client.load_root_node_full()) as NodeRaw;

  return {
    content: jsNode.content,
    edges: jsNode.edges.map(({ label, dest_idx }) => ({
      label,
      dest: dest_idx,
    })),
  };
}

/**
 * Fetch any node by its numeric index.
 */
export async function fetchNodeFull(
  nodeIdx: number
): Promise<{
  content: string;
  edges: { label: string; dest: number }[];
}> {
  const client = await getClient();
  const jsNode = (await client.load_node_full(nodeIdx)) as NodeRaw;
  return {
    content: jsNode.content,
    edges: jsNode.edges.map(({ label, dest_idx }) => ({
      label,
      dest: dest_idx,
    })),
  };
}

