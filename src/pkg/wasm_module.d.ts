/* tslint:disable */
/* eslint-disable */
/**
 * Entry point invoked by `wasm_bindgen` when the module is instantiated.
 *
 * Installs the panic hook so that any Rust panics are forwarded to the
 * browser console as `console.error` messages, improving runtime
 * diagnostics when using the module from JavaScript.
 */
export function __wasm_start(): void;
/**
 * The main game loader exposed to JavaScript via wasm_bindgen.
 * Handles probing, range-requests, parsing TLV, zstd decompression,
 * and exposes `load_root_node_full` / `load_node_full` APIs.
 */
export class CyoaGame {
  free(): void;
  /**
   * Constructs a new `CyoaGame` instance by probing the remote file
   * at `path` for its total size and HTTP Range support, then fetching
   * and parsing the on‐disk index.
   *
   * # Parameters
   *
   * - `path`: URL or filesystem path (relative to the site root) of
   *   the `.cyoa` binary file.
   *
   * # Returns
   *
   * - `Ok(CyoaGame)`: if the file was probed successfully and its index
   *   parsed without error.
   * - `Err(JsValue)`: if there was any HTTP error, missing range support,
   *   invalid magic, out‐of‐range index pointer, or parse failure.
   *
   * # Examples
   *
   * ```ignore
   * // In JavaScript:
   * const game = await new CyoaGame("/games/mystory.cy");
   * ```
   */
  constructor(path: string);
  /**
   * Returns a JavaScript `Array` of all chunk IDs present in the file’s
   * parsed index, formatted as uppercase hex strings.
   *
   * Each entry is the 3‐byte chunk identifier, e.g. `"000102"`.
   *
   * # Examples
   *
   * ```ignore
   * let ids = game.chunk_ids();              // ["000001", "000002", …]
   * console.log(ids[0]);                     // "000001"
   * ```
   */
  chunk_ids(): Array<any>;
  /**
   * Loads the node at the given index (into the parsed index vector),
   * fully fetching its content text and all outgoing edges—with labels
   * and destination indices—all in one batched request (wherever possible).
   *
   * # Parameters
   *
   * - `idx`: Zero‐based index into the game’s index entries. Must point
   *   at a `ChunkType::Node` entry.
   *
   * # Returns
   *
   * - `Ok(JsValue)`: A JS object with shape `{ content: string, edges: Array< { label: string, dest_idx: number } > }`.
   * - `Err(JsValue)`: If `idx` is out of range, not a node chunk, or any
   *   network/parse error occurs.
   *
   * # Errors
   *
   * - `GameError::Parse("not a node chunk")` if the indexed entry isn’t a node.
   * - `GameError::Http` if any range‐request fails.
   * - `GameError::Parse(...)` for TLV or decompression failures.
   *
   * # Examples
   *
   * ```ignore
   * let node = await game.load_node_full(3);
   * console.log(node.content);               // "You stand at a crossroads..."
   * console.log(node.edges.length);          // e.g. 2
   * ```
   */
  load_node_full(idx: number): Promise<any>;
  /**
   * Loads the “root” node as specified by the metadata chunk
   * `ID_ROOT_POINTER`. This is equivalent to finding the metadata
   * entry whose ID is `[0,0,1]`, reading its value as a node‐chunk
   * ID, and then calling `load_node_full` on that node’s index.
   *
   * # Returns
   *
   * - `Ok(JsValue)`: The same structured object as `load_node_full`.
   * - `Err(JsValue)`: If the metadata chunk is missing, invalid, or any
   *   subsequent fetch/parse fails.
   *
   * # Errors
   *
   * - `GameError::MissingRoot` if no metadata chunk with ID `[0,0,1]` is found.
   * - All other errors are forwarded from `load_node_full`.
   *
   * # Examples
   *
   * ```ignore
   * let root = await game.load_root_node_full();
   * console.log(root.content);               // The starting passage text
   * ```
   */
  load_root_node_full(): Promise<any>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_cyoagame_free: (a: number, b: number) => void;
  readonly cyoagame_new: (a: number, b: number) => any;
  readonly cyoagame_chunk_ids: (a: number) => any;
  readonly cyoagame_load_node_full: (a: number, b: number) => any;
  readonly cyoagame_load_root_node_full: (a: number) => any;
  readonly __wasm_start: () => void;
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
  readonly closure60_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure73_externref_shim: (a: number, b: number, c: any, d: any) => void;
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
