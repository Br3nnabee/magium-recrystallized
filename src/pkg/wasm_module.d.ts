/* tslint:disable */
/* eslint-disable */
export function __wasm_start(): void;
export class CyoaGame {
  free(): void;
  /**
   * Async constructor
   */
  constructor(path: string);
  /**
   * Return all chunk IDs as hex strings
   */
  chunk_ids(): Array<any>;
  /**
   * Async: get root node chunk ID hex
   */
  get_root_node(): Promise<string>;
  /**
   * Async: fetch content chunk text
   */
  get_content(idx: number): Promise<string>;
  /**
   * Async: fetch node->content translation ID
   */
  get_node_content(idx: number): Promise<string>;
  get_edges(idx: number): Promise<Array<any>>;
  get_edge_label(idx: number): Promise<string>;
  /**
   * Fetch and return the hex ID of the node this edge points to
   */
  get_edge_destination(idx: number): Promise<string>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wasm_start: () => void;
  readonly __wbg_cyoagame_free: (a: number, b: number) => void;
  readonly cyoagame_new: (a: number, b: number) => any;
  readonly cyoagame_chunk_ids: (a: number) => any;
  readonly cyoagame_get_root_node: (a: number) => any;
  readonly cyoagame_get_content: (a: number, b: number) => any;
  readonly cyoagame_get_node_content: (a: number, b: number) => any;
  readonly cyoagame_get_edges: (a: number, b: number) => any;
  readonly cyoagame_get_edge_label: (a: number, b: number) => any;
  readonly cyoagame_get_edge_destination: (a: number, b: number) => any;
  readonly rust_zstd_wasm_shim_qsort: (a: number, b: number, c: number, d: number) => void;
  readonly rust_zstd_wasm_shim_malloc: (a: number) => number;
  readonly rust_zstd_wasm_shim_memcmp: (a: number, b: number, c: number) => number;
  readonly rust_zstd_wasm_shim_calloc: (a: number, b: number) => number;
  readonly rust_zstd_wasm_shim_free: (a: number) => void;
  readonly rust_zstd_wasm_shim_memcpy: (a: number, b: number, c: number) => number;
  readonly rust_zstd_wasm_shim_memmove: (a: number, b: number, c: number) => number;
  readonly rust_zstd_wasm_shim_memset: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly closure72_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure84_externref_shim: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
