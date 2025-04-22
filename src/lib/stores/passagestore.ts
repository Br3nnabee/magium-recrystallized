import { writable, derived, get, type Readable } from 'svelte/store';
import {
  fetchChunkIds,
  fetchRootIndex,
  fetchContent,
  fetchEdges,
  fetchEdgeLabel,
  fetchEdgeDestination
} from '$lib/wasm';

interface EdgeInfo {
  label: string;
  dest: number;
}
export interface StoryNode {
  content: string;
  edges: EdgeInfo[];
}
interface Metadata {
  chunkIds: string[];
  root: number;
}

export const metadata = writable<Metadata | null>(null);
export const nodeCache = writable<Map<number, StoryNode>>(new Map());
export const currentIndex = writable<number>(0);

// Internally store the canonical URL for the story file
let storyUrl: string;

/**
 * Convert a relative path to a full URL using import.meta.url
 */
function canonicalize(path: string): string {
  // If path is a root-relative or absolute URL, use it directly
  if (path.startsWith('/') || /^https?:\/\//.test(path)) {
    return path;
  }
  // Otherwise assume it resides in the static directory at web root
  return `/${path}`;
}


/**
 * Always return the current node (content + edges), or a loading stub.
 */
export const currentNode: Readable<StoryNode> = derived(
  [metadata, nodeCache, currentIndex],
  ([$meta, $cache, $idx]) => {
    if (!$meta) {
      return { content: 'Initializing…', edges: [] };
    }
    const existing = $cache.get($idx);
    if (existing) return existing;
    return { content: 'Loading…', edges: [] };
  }
);

/**
 * Initialize the story: load metadata, set root as current, and preload root node.
 */
export async function initialize(path: string): Promise<void> {
  storyUrl = canonicalize(path);
  try {
    const [ids, root] = await Promise.all([
      fetchChunkIds(storyUrl),
      fetchRootIndex(storyUrl)
    ]);
    metadata.set({ chunkIds: ids, root });
    currentIndex.set(root);
    // preload the root node
    loadNode(root);
  } catch (e) {
    console.error('initialize error:', e);
    metadata.set({ chunkIds: [], root: 0 });
    currentIndex.set(0);
    nodeCache.set(new Map([[0, { content: `Error: ${e}`, edges: [] }]]));
  }
}

/**
 * Load a node (content + edges) if not already cached.
 */
export async function loadNode(idx: number): Promise<void> {
  const $cache = get(nodeCache);
  if ($cache.has(idx)) return;
  const $meta = get(metadata);
  if (!$meta) throw new Error('Story not initialized');

  // fetch content
  const content = await fetchContent(storyUrl, idx);

  // fetch edge hex IDs
  const edgeHexes = await fetchEdges(storyUrl, idx);

  // resolve edges
  const edges: EdgeInfo[] = await Promise.all(
    edgeHexes.map(async hex => {
      const eIdx = $meta.chunkIds.findIndex(id => id === hex);
      if (eIdx < 0) return { label: `<missing:${hex}>`, dest: -1 };
      const [label, destHex] = await Promise.all([
        fetchEdgeLabel(storyUrl, eIdx),
        fetchEdgeDestination(storyUrl, eIdx)
      ]);
      const dest = $meta.chunkIds.findIndex(id => id === destHex);
      return { label, dest: dest >= 0 ? dest : -1 };
    })
  );

  // cache the node
  const newMap = new Map($cache);
  newMap.set(idx, { content, edges });
  nodeCache.set(newMap);
}

/**
 * Navigate to a node index and load it.
 */
export function goTo(idx: number): void {
  currentIndex.set(idx);
  loadNode(idx);
}
