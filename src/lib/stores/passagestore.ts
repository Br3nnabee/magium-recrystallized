import { writable, derived, get } from 'svelte/store';
import {
  fetchChunkIds,
  fetchNodePassage,
  fetchEdges,
  fetchEdgeLabel,
  fetchEdgeDestination,
  fetchRootIndex
} from '$lib/wasm';

interface EdgeInfo {
  label: string;
  dest: number;
}

export interface StoryNode {
  content: string;
  edges: EdgeInfo[];
}

export const metadata = writable<{ chunkIds: string[]; root: number } | null>(null);
export const nodeCache = writable<Map<number, StoryNode>>(new Map());
export const currentIndex = writable<number>(0);

export const currentNode = derived(
  [metadata, nodeCache, currentIndex],
  ([$meta, $cache, $idx]) => {
    if (!$meta) return { content: 'Initializing…', edges: [] };
    return $cache.get($idx) ?? { content: 'Loading…', edges: [] };
  }
);

export async function initialize(): Promise<void> {
  try {
    const [ids, root] = await Promise.all([
      fetchChunkIds(),
      fetchRootIndex()
    ]);
    metadata.set({ chunkIds: ids, root });
    currentIndex.set(root);
    await loadNode(root);
  } catch (e) {
    console.error('initialize error:', e);
    metadata.set(null);
    nodeCache.set(new Map([[0, { content: `Error: ${e}`, edges: [] }]]));
    currentIndex.set(0);
  }
}

function cacheError(idx: number, msg: string) {
  nodeCache.update(cache => {
    const m = new Map(cache);
    m.set(idx, { content: msg, edges: [] });
    return m;
  });
}

export async function loadNode(idx: number): Promise<void> {
  const cache = get(nodeCache);
  if (cache.has(idx)) return;

  const meta = get(metadata);
  if (!meta) throw new Error('Story not initialized');

  // 1) fetch the correct text for this node
  let content: string;
  try {
    content = await fetchNodePassage(idx);
  } catch (e) {
    console.error(`fetchNodePassage failed for idx=${idx}:`, e);
    return cacheError(idx, `Error loading passage #${idx}: ${e}`);
  }

  // 2) fetch outgoing edge IDs
  let edgeHexes: string[];
  try {
    edgeHexes = await fetchEdges(idx);
  } catch (e) {
    console.error(`fetchEdges failed for idx=${idx}:`, e);
    return cacheError(idx, `Error loading edges for #${idx}: ${e}`);
  }

  // 3) resolve each edge to {label, destIdx}
  const edges: EdgeInfo[] = [];
  for (const hex of edgeHexes) {
    const edgeIdx = meta.chunkIds.findIndex(id => id === hex);
    if (edgeIdx < 0) {
      edges.push({ label: `<missing:${hex}>`, dest: -1 });
      continue;
    }

    let label: string;
    try {
      label = await fetchEdgeLabel(edgeIdx);
    } catch {
      label = `<error-label:${hex}>`;
    }

    let destHex: string;
    try {
      destHex = await fetchEdgeDestination(edgeIdx);
    } catch {
      destHex = '';
    }
    const destIdx = meta.chunkIds.findIndex(id => id === destHex);
    edges.push({ label, dest: destIdx >= 0 ? destIdx : -1 });
  }

  // 4) cache
  nodeCache.update(cache => {
    const m = new Map(cache);
    m.set(idx, { content, edges });
    return m;
  });
}

export function goTo(idx: number): void {
  currentIndex.set(idx);
  loadNode(idx);
}
