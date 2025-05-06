use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use js_sys::{Array, Uint8Array};
use web_sys::{Headers, RequestInit, RequestMode, Response, Window, window};
use futures::future::try_join_all;
use serde_wasm_bindgen::to_value;
use std::cell::RefCell;
use std::sync::Arc;

use crate::types::{ChunkType, IndexEntry, ContentEntry, EdgeOutput, NodeOutput};
use crate::tlv::{
    parse_header,
    parse_index,
    parse_tlv_header,
    decompress_payload,
    parse_node_content_seq,
    parse_node_edges_ids,
    parse_content_text,
    parse_edge_label_dest_cids,
};
use crate::http::{probe_range, fetch_range};
use crate::cache::{RawCache, get_raw_chunk};
use crate::utils::GameError;
use crate::wasmtable::run_guard;

/// Number of bytes in the fixed CYOA header.
const HEADER_LEN: usize = 22;
/// ID used in metadata to point to the root node.
const ID_ROOT_POINTER: [u8; 3] = [0, 0, 1];

/// The main game loader exposed to JavaScript via wasm_bindgen.
/// Handles probing, range-requests, parsing TLV, zstd decompression,
/// and exposes `load_root_node_full` / `load_node_full` APIs.
#[wasm_bindgen]
pub struct CyoaGame {
    url: String,
    size: u64,
    supports_range: bool,
    index: Vec<IndexEntry>,
    raw_cache: RefCell<RawCache>,
}

#[wasm_bindgen]
impl CyoaGame {
    /// Constructs a new `CyoaGame` instance by probing the remote file
    /// at `path` for its total size and HTTP Range support, then fetching
    /// and parsing the on‐disk index.
    #[wasm_bindgen(constructor)]
    pub async fn new(path: String) -> Result<CyoaGame, JsValue> {
        let url = if path.starts_with('/') {
            path.clone()
        } else {
            format!("/{}", path)
        };
        let win = window().ok_or(GameError::Other("No window object".to_string())).map_err(JsValue::from)?;
        let (size, supports) = probe_range(&win, &url).await.map_err(JsValue::from)?;

        let header = if supports {
            fetch_range(&win, &url, 0, Some((HEADER_LEN - 1) as u64)).await?
        } else {
            return Err(GameError::RangeNotSupported.into());
        };
        let index_offset = parse_header(&header)?;

        if index_offset >= size {
            return Err(GameError::IndexOutOfRange.into());
        }
        let idx_blob = fetch_range(&win, &url, index_offset, None).await?;

        let index = parse_index(&idx_blob).map_err(JsValue::from)?;
        Ok(CyoaGame {
            url,
            size,
            supports_range: supports,
            index,
            raw_cache: RefCell::new(RawCache::new(100)),
        })
    }

    /// Returns a JavaScript `Array` of all chunk IDs present in the file’s
    /// parsed index, formatted as uppercase hex strings.
    #[wasm_bindgen]
    pub fn chunk_ids(&self) -> Array {
        let arr = Array::new();
        for e in &self.index {
            let s = e.chunk_id.iter().map(|b| format!("{:02X}", b)).collect::<String>();
            arr.push(&JsValue::from_str(&s));
        }
        arr
    }

    /// Loads the node at the given index (into the parsed index vector),
    /// fully fetching its content text and all outgoing edges—with labels
    /// and destination indices—all in one batched request.
    #[wasm_bindgen]
    pub async fn load_node_full(&self, idx: usize) -> Result<JsValue, JsValue> {
        // 1) Validate and fetch raw node chunk
        let entry = self.index.get(idx)
            .ok_or(GameError::Parse("node index out of range"))
            .map_err(JsValue::from)?;
        if entry.chunk_type != ChunkType::Node {
            return Err(GameError::Parse("not a node chunk").into());
        }
        let raw_node = get_raw_chunk(&self.raw_cache, self.supports_range, &self.url, entry).await?;

        // 2) Parse TLV header and decompress node payload
        let (_t, _id, flags, comp_len, un_len_opt, hdr_len) =
            parse_tlv_header(&raw_node).map_err(JsValue::from)?;
        let payload = decompress_payload(
            flags,
            &raw_node[hdr_len..hdr_len + comp_len as usize],
            un_len_opt,
        ).map_err(JsValue::from)?;

        // 3) Parse content sequence entries (with guards)
        let seq_entries = parse_node_content_seq(&payload).map_err(JsValue::from)?;

        // 4) Run guards and collect content IDs to include
        let mut wanted_ids = Vec::new();
        for entry in seq_entries {
            if let Some((func_id, guard_bytes)) = entry.guard {
                if !run_guard(func_id, &guard_bytes) {
                    continue;
                }
            }
            wanted_ids.push(entry.content_id);
        }

        // 5) Find index entries for the surviving content IDs
        let content_indexes: Vec<&IndexEntry> = wanted_ids
            .iter()
            .map(|cid| {
                self.index
                    .iter()
                    .find(|e| e.chunk_type == ChunkType::Content && e.chunk_id == *cid)
                    .ok_or(GameError::Parse("content chunk not found"))
            })
            .collect::<Result<_, _>>()
            .map_err(JsValue::from)?;

        // 6) Fetch all content chunks in parallel
        let raw_contents = try_join_all(
            content_indexes.iter().map(|e| get_raw_chunk(&self.raw_cache, self.supports_range, &self.url, e))
        ).await.map_err(JsValue::from)?;

        // 7) Decompress & parse each content text, concatenate
        let mut full_text = String::new();
        for raw_c in raw_contents {
            let (_t2, _i2, f2, c2, u2_opt, h2) = parse_tlv_header(&raw_c).map_err(JsValue::from)?;
            let pl = decompress_payload(f2, &raw_c[h2..h2 + c2 as usize], u2_opt).map_err(JsValue::from)?;
            let txt = parse_content_text(&pl).map_err(JsValue::from)?;
            full_text.push_str(&txt);
        }

        // 8) Edge parsing
        let edge_cids = parse_node_edges_ids(&payload).map_err(JsValue::from)?;
        let edge_entries: Vec<&IndexEntry> = edge_cids
            .iter()
            .map(|cid| {
                self.index
                    .iter()
                    .find(|e| e.chunk_type == ChunkType::Edge && e.chunk_id == *cid)
                    .ok_or(GameError::Parse("edge chunk not found"))
            })
            .collect::<Result<_, _>>()
            .map_err(JsValue::from)?;
        let raw_edges = try_join_all(
            edge_entries.iter().map(|e| get_raw_chunk(&self.raw_cache, self.supports_range, &self.url, e))
        ).await.map_err(JsValue::from)?;

        let mut edge_meta = Vec::with_capacity(edge_entries.len());
        for raw_e in raw_edges {
            let (_t3, _i3, f3, c3, u3_opt, h3) = parse_tlv_header(&raw_e).map_err(JsValue::from)?;
            let pl = decompress_payload(f3, &raw_e[h3..h3 + c3 as usize], u3_opt).map_err(JsValue::from)?;
            let (label_cid, dest_cid) = parse_edge_label_dest_cids(&pl).map_err(JsValue::from)?;
            edge_meta.push((label_cid, dest_cid));
        }

        let label_entries: Vec<&IndexEntry> = edge_meta
            .iter()
            .map(|(lc, _)| {
                self.index
                    .iter()
                    .find(|e| e.chunk_type == ChunkType::Content && &e.chunk_id == lc)
                    .ok_or(GameError::Parse("label content not found"))
            })
            .collect::<Result<_, _>>()
            .map_err(JsValue::from)?;
        let raw_labels = try_join_all(
            label_entries.iter().map(|e| get_raw_chunk(&self.raw_cache, self.supports_range, &self.url, e))
        ).await.map_err(JsValue::from)?;
        let mut edges_out = Vec::with_capacity(edge_meta.len());
        for (raw_lbl, (_, dest_cid)) in raw_labels.into_iter().zip(edge_meta) {
            let (_t4, _i4, f4, c4, u4_opt, h4) = parse_tlv_header(&raw_lbl).map_err(JsValue::from)?;
            let pl = decompress_payload(f4, &raw_lbl[h4..h4 + c4 as usize], u4_opt).map_err(JsValue::from)?;
            let label_text = parse_content_text(&pl).map_err(JsValue::from)?;
            let dest_idx = self.index
                .iter()
                .position(|e| e.chunk_type == ChunkType::Node && e.chunk_id == dest_cid)
                .ok_or(GameError::Parse("edge destination node not found"))
                .map_err(JsValue::from)?;
            edges_out.push(EdgeOutput { label: label_text, dest_idx: dest_idx as u32 });
        }

        // 9) Serialize and return
        let node = NodeOutput { content: full_text, edges: edges_out };
        to_value(&node).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    /// Loads the “root” node as specified by the metadata chunk ID_ROOT_POINTER.
    #[wasm_bindgen]
    pub async fn load_root_node_full(&self) -> Result<JsValue, JsValue> {
        let meta_idx = self.index.iter()
            .position(|e| e.chunk_type == ChunkType::Metadata && e.chunk_id == ID_ROOT_POINTER)
            .ok_or(GameError::MissingRoot)
            .map_err(JsValue::from)?;
        let entry = &self.index[meta_idx];
        let raw = get_raw_chunk(&self.raw_cache, self.supports_range, &self.url, entry).await?;
        let (_t, _i, _f, _c, _u, h) = parse_tlv_header(&raw).map_err(JsValue::from)?;
        let mut cid = [0u8; 3];
        cid.copy_from_slice(&raw[h..h + 3]);
        let node_idx = self.index.iter()
            .position(|e| e.chunk_type == ChunkType::Node && e.chunk_id == cid)
            .ok_or(GameError::Parse("root node chunk not found"))
            .map_err(JsValue::from)?;
        self.load_node_full(node_idx).await
    }
}
