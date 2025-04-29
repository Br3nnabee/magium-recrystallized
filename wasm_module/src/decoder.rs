use byteorder::{LittleEndian, ReadBytesExt};
use futures::future::try_join_all;
use js_sys::{Array, Uint8Array};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, RequestInit, RequestMode, Response, Window, window};
use zstd_safe::decompress;

// -- Type-safe enums and structured errors --
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChunkType {
    Node = 0x01,
    Edge = 0x02,
    Content = 0x03,
    Metadata = 0x04,
    ArgBlobPool = 0xFD,
    WasmTable = 0xFE,
}

#[derive(Debug)]
enum GameError {
    Http(u16),
    RangeNotSupported,
    InvalidMagic,
    IndexOutOfRange,
    Parse(&'static str),
    MissingRoot,
    Other(String),
}

impl From<GameError> for JsValue {
    fn from(err: GameError) -> JsValue {
        match err {
            GameError::Http(code) => JsValue::from_str(&format!("HTTP error: {}", code)),
            GameError::InvalidMagic => JsValue::from_str("Invalid file magic"),
            GameError::IndexOutOfRange => JsValue::from_str("Index out of range"),
            GameError::RangeNotSupported => JsValue::from_str("Range requests not supported"),
            GameError::MissingRoot => JsValue::from_str("Root pointer metadata missing"),
            GameError::Parse(msg) => JsValue::from_str(msg),
            GameError::Other(s) => JsValue::from_str(&s),
        }
    }
}

// -- Configurable logging --
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        if cfg!(debug_assertions) {
            web_sys::console::log_1(&JsValue::from_str(&format!($($arg)*)));
        }
    }};
}

const HEADER_LEN: usize = 22;
const ID_ROOT_POINTER: [u8; 3] = [0, 0, 1];

#[derive(Clone, Debug)]
struct IndexEntry {
    chunk_type: ChunkType,
    chunk_id: [u8; 3],
    offset: u64,
    length: u32,
}

#[derive(Serialize)]
struct EdgeOutput {
    label: String,
    dest_idx: u32,
}

#[derive(Serialize)]
struct NodeOutput {
    content: String,
    edges: Vec<EdgeOutput>,
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return Vec::new();
    }
    // sort by start
    ranges.sort_unstable_by_key(|&(s, _)| s);
    let mut merged = Vec::new();
    let (mut cur_start, mut cur_end) = ranges[0];
    for &(s, e) in &ranges[1..] {
        if s <= cur_end.checked_add(1).unwrap_or(cur_end) {
            // overlapping or contiguous
            cur_end = cur_end.max(e);
        } else {
            merged.push((cur_start, cur_end));
            cur_start = s;
            cur_end = e;
        }
    }
    merged.push((cur_start, cur_end));
    merged
}

// LRU cache for raw chunks with Arc<Vec<u8>>
struct RawCache {
    entries: VecDeque<([u8; 3], Arc<Vec<u8>>)>,
    capacity: usize,
}

impl RawCache {
    fn new(cap: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            capacity: cap,
        }
    }
    fn get(&mut self, key: &[u8; 3]) -> Option<Arc<Vec<u8>>> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
            let (k, v) = self.entries.remove(pos).unwrap();
            self.entries.push_front((k, v.clone()));
            return Some(v);
        }
        None
    }
    fn insert(&mut self, key: [u8; 3], value: Arc<Vec<u8>>) {
        if self.entries.len() == self.capacity {
            self.entries.pop_back();
        }
        self.entries.push_front((key, value));
    }
}

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
    #[wasm_bindgen(constructor)]
    pub async fn new(path: String) -> Result<CyoaGame, JsValue> {
        let url = if path.starts_with('/') {
            path.clone()
        } else {
            format!("/{}", path)
        };
        let win = window().ok_or(GameError::Other("No window object".to_string()))?;
        let (size, supports) = Self::probe_range(&win, &url).await.map_err(JsValue::from)?;
        let header = if supports {
            Self::fetch_range(&win, &url, 0, Some((HEADER_LEN - 1) as u64)).await?
        } else {
            return Err(GameError::RangeNotSupported.into());
        };
        let index_offset = Self::parse_header(&header)?;

        if index_offset >= size {
            return Err(GameError::IndexOutOfRange.into());
        }
        let idx_blob = Self::fetch_range(&win, &url, index_offset, None).await?;

        let index = Self::parse_index(&idx_blob).map_err(JsValue::from)?;
        Ok(CyoaGame {
            url,
            size,
            supports_range: supports,
            index,
            raw_cache: RefCell::new(RawCache::new(100)),
        })
    }

    #[wasm_bindgen]
    pub fn chunk_ids(&self) -> Array {
        let arr = Array::new();
        for e in &self.index {
            let s = e
                .chunk_id
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<String>();
            arr.push(&JsValue::from_str(&s));
        }
        arr
    }

    #[wasm_bindgen]
    pub async fn load_node_full(&self, idx: usize) -> Result<JsValue, JsValue> {
        // 1) Validate and fetch raw node chunk
        let entry = self
            .index
            .get(idx)
            .ok_or(GameError::Parse("node index out of range"))
            .map_err(JsValue::from)?;
        if entry.chunk_type != ChunkType::Node {
            return Err(GameError::Parse("not a node chunk").into());
        }
        let raw_node = self.get_raw_chunk(entry).await?;

        // 2) Parse TLV and decompress
        let (_t, _id, flags, comp_len, un_len, hdr_len) =
            Self::parse_tlv_header(&raw_node).map_err(JsValue::from)?;
        let payload = Self::decompress_payload(
            flags,
            &raw_node[hdr_len..hdr_len + comp_len as usize],
            un_len,
        )
        .map_err(JsValue::from)?;

        // 3) Extract content & edge CIDs
        let content_cid = Self::parse_node_content_cid(&payload).map_err(JsValue::from)?;
        let edge_cids = Self::parse_node_edges_ids(&payload).map_err(JsValue::from)?;

        // 4) Locate index entries
        let content_entry = self
            .index
            .iter()
            .find(|e| e.chunk_type == ChunkType::Content && e.chunk_id == content_cid)
            .ok_or(GameError::Parse("content chunk not found"))
            .map_err(JsValue::from)?;
        let edge_entries: Vec<_> = edge_cids
            .iter()
            .filter_map(|cid| {
                self.index
                    .iter()
                    .find(|e| e.chunk_type == ChunkType::Edge && &e.chunk_id == cid)
            })
            .collect();

        // 5) Build ranges (merge inside fetch_ranges)
        let mut ranges = Vec::with_capacity(1 + edge_entries.len());
        ranges.push((
            content_entry.offset,
            content_entry.offset + content_entry.length as u64 - 1,
        ));
        for e in &edge_entries {
            ranges.push((e.offset, e.offset + e.length as u64 - 1));
        }

        // 6) Fetch all parts in parallel (fetch_ranges merges)
        let win = window().ok_or(GameError::Other("no window".to_string()))?;
        let parts = self.fetch_ranges(&win, &self.url, &ranges).await?;

        // 7) Parse node content text
        let content_text = {
            let raw_c = &parts[0];
            let (_t2, _i2, f2, c2, u2, h2) =
                Self::parse_tlv_header(raw_c).map_err(JsValue::from)?;
            let pl = Self::decompress_payload(f2, &raw_c[h2..h2 + c2 as usize], u2)
                .map_err(JsValue::from)?;
            Self::parse_content_text(&pl).map_err(JsValue::from)?
        };

        // 8) Batch-parse edge metas
        let mut edge_meta = Vec::with_capacity(edge_entries.len());
        for raw_e in parts.iter().skip(1) {
            let (_t3, _i3, f3, c3, u3, h3) =
                Self::parse_tlv_header(raw_e).map_err(JsValue::from)?;
            let pl = Self::decompress_payload(f3, &raw_e[h3..h3 + c3 as usize], u3)
                .map_err(JsValue::from)?;
            let (label_cid, dest_cid) =
                Self::parse_edge_label_dest_cids(&pl).map_err(JsValue::from)?;
            edge_meta.push((label_cid, dest_cid));
        }

        // 9) Fetch and parse labels
        let label_entries: Vec<&IndexEntry> = edge_meta
            .iter()
            .map(|(lc, _)| {
                self.index
                    .iter()
                    .find(|e| e.chunk_type == ChunkType::Content && &e.chunk_id == lc)
                    .ok_or(GameError::Parse("label content not found"))
            })
            .collect::<Result<_, GameError>>()
            .map_err(JsValue::from)?;

        let raw_labels = try_join_all(label_entries.iter().map(|e| self.get_raw_chunk(e))).await?;

        // 10) Build EdgeOutput list
        let mut edges_out = Vec::with_capacity(edge_entries.len());
        for (raw_lbl, (_, dest_cid)) in raw_labels.into_iter().zip(edge_meta) {
            let (_t4, _i4, f4, c4, u4, h4) =
                Self::parse_tlv_header(&raw_lbl).map_err(JsValue::from)?;
            let pl = Self::decompress_payload(f4, &raw_lbl[h4..h4 + c4 as usize], u4)
                .map_err(JsValue::from)?;
            let label_text = Self::parse_content_text(&pl).map_err(JsValue::from)?;
            let dest_idx = self
                .index
                .iter()
                .position(|e| e.chunk_type == ChunkType::Node && e.chunk_id == dest_cid)
                .ok_or(GameError::Parse("edge destination node not found"))
                .map_err(JsValue::from)?;
            edges_out.push(EdgeOutput {
                label: label_text,
                dest_idx: dest_idx as u32,
            });
        }

        // 11) Serialize and return
        let node = NodeOutput {
            content: content_text,
            edges: edges_out,
        };
        to_value(&node).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen]
    pub async fn load_root_node_full(&self) -> Result<JsValue, JsValue> {
        let meta_idx = self
            .index
            .iter()
            .position(|e| e.chunk_type == ChunkType::Metadata && e.chunk_id == ID_ROOT_POINTER)
            .ok_or(GameError::MissingRoot)
            .map_err(JsValue::from)?;
        let entry = &self.index[meta_idx];
        let raw = self.get_raw_chunk(entry).await?;
        let (_t, _i, _f, _c, _u, h) = Self::parse_tlv_header(&raw).map_err(JsValue::from)?;
        let mut cid = [0u8; 3];
        cid.copy_from_slice(&raw[h..h + 3]);
        let node_idx = self
            .index
            .iter()
            .position(|e| e.chunk_type == ChunkType::Node && e.chunk_id == cid)
            .ok_or(GameError::Parse("root node chunk not found"))
            .map_err(JsValue::from)?;
        self.load_node_full(node_idx).await
    }

    // -- HTTP & parsing helpers --
    async fn probe_range(win: &Window, url: &str) -> Result<(u64, bool), GameError> {
        let mut init = RequestInit::new();
        init.set_method("GET");
        init.set_mode(RequestMode::SameOrigin);
        let mut hdrs = Headers::new().map_err(|e| GameError::Other(format!("{:?}", e)))?;
        hdrs.append("Range", "bytes=0-0")
            .map_err(|e| GameError::Other(format!("{:?}", e)))?;
        init.set_headers(&hdrs.into());
        let resp = JsFuture::from(win.fetch_with_str_and_init(url, &init))
            .await
            .map_err(|_| GameError::Http(0))?
            .dyn_into::<Response>()
            .map_err(|_| GameError::Other("Invalid response".to_string()))?;
        let status = resp.status();
        let ranged = status == 206;
        let size = if ranged {
            let cr = resp
                .headers()
                .get("content-range")
                .map_err(|_| GameError::Other("No Content-Range".to_string()))?
                .ok_or(GameError::Other("Bad Content-Range".to_string()))?;
            cr.split('/')
                .nth(1)
                .ok_or(GameError::Other("Bad Content-Range".to_string()))?
                .parse()
                .map_err(|_| GameError::Other("Bad Content-Range".to_string()))?
        } else {
            let cl = resp
                .headers()
                .get("content-length")
                .map_err(|_| GameError::Other("No Content-Length".to_string()))?
                .ok_or(GameError::Other("Bad Content-Length".to_string()))?;
            cl.parse()
                .map_err(|_| GameError::Other("Bad Content-Length".to_string()))?
        };
        Ok((size, ranged))
    }
    async fn fetch_range(
        win: &Window,
        url: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Vec<u8>, JsValue> {
        let mut init = RequestInit::new();
        init.set_method("GET");
        init.set_mode(RequestMode::SameOrigin);
        let mut hdrs = Headers::new().map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let range = match end {
            Some(e) => format!("bytes={}-{}", start, e),
            None => format!("bytes={}-", start),
        };
        hdrs.append("Range", &range)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        init.set_headers(&hdrs.into());
        let resp = JsFuture::from(win.fetch_with_str_and_init(url, &init))
            .await?
            .dyn_into::<Response>()?;
        if !resp.ok() {
            return Err(GameError::Http(resp.status()).into());
        }
        let buf = JsFuture::from(resp.array_buffer()?).await?;
        let arr = Uint8Array::new(&buf);
        let mut v = vec![0; arr.length() as usize];
        arr.copy_to(&mut v);
        Ok(v)
    }
    fn parse_header(header: &[u8]) -> Result<u64, GameError> {
        let mut c = Cursor::new(header);
        let mut magic = [0; 4];
        c.read_exact(&mut magic)
            .map_err(|_| GameError::InvalidMagic)?;
        if &magic != b"CYOA" {
            return Err(GameError::InvalidMagic);
        }
        c.seek(SeekFrom::Current(10))
            .map_err(|_| GameError::Parse("Seek error"))?;
        c.read_u64::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read u64 error"))
    }

    fn parse_index(blob: &[u8]) -> Result<Vec<IndexEntry>, GameError> {
        let mut c = Cursor::new(blob);
        let cnt = c
            .read_u32::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read u32 error"))?;
        let mut out = Vec::with_capacity(cnt as usize);
        for _ in 0..cnt {
            let t = c.read_u8().map_err(|_| GameError::Parse("Read u8"))?;

            let chunk_type = match t {
                0x01 => ChunkType::Node,
                0x02 => ChunkType::Edge,
                0x03 => ChunkType::Content,
                0x04 => ChunkType::Metadata,
                0xFD => ChunkType::ArgBlobPool,
                0xFE => ChunkType::WasmTable,
                _ => return Err(GameError::Parse("Unknown chunk type")),
            };

            let mut id = [0; 3];
            c.read_exact(&mut id)
                .map_err(|_| GameError::Parse("Read id"))?;
            let off = c
                .read_u64::<LittleEndian>()
                .map_err(|_| GameError::Parse("Read offset"))?;
            let len = c
                .read_u32::<LittleEndian>()
                .map_err(|_| GameError::Parse("Read length"))?;
            out.push(IndexEntry {
                chunk_type,
                chunk_id: id,
                offset: off,
                length: len,
            });
        }
        Ok(out)
    }

    fn parse_tlv_header(
        raw: &[u8],
    ) -> Result<(u8, [u8; 3], u8, u32, Option<u32>, usize), GameError> {
        let mut c = Cursor::new(raw);
        let t = c.read_u8().map_err(|_| GameError::Parse("Read type"))?;
        let mut id = [0; 3];
        c.read_exact(&mut id)
            .map_err(|_| GameError::Parse("Read id"))?;
        let flags = c.read_u8().map_err(|_| GameError::Parse("Read flags"))?;
        let comp = c
            .read_u32::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read comp len"))?;
        let mut hlen = 1 + 3 + 1 + 4;
        let un = if flags & 1 != 0 {
            let u = c
                .read_u32::<LittleEndian>()
                .map_err(|_| GameError::Parse("Read unlen"))?;
            hlen += 4;
            Some(u)
        } else {
            None
        };
        Ok((t, id, flags, comp, un, hlen))
    }

    fn decompress_payload(flags: u8, data: &[u8], un: Option<u32>) -> Result<Vec<u8>, GameError> {
        if flags & 1 != 0 {
            let target = un.ok_or(GameError::Parse("Missing uncompressed length"))? as usize;
            let mut out = vec![0u8; target];
            let written = decompress(out.as_mut_slice(), data)
                .map_err(|e| GameError::Other(e.to_string()))?;
            out.truncate(written);
            Ok(out)
        } else {
            Ok(data.to_vec())
        }
    }

    fn parse_node_content_cid(data: &[u8]) -> Result<[u8; 3], GameError> {
        let mut c = Cursor::new(data);
        let id_len = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read id_len"))?;
        c.seek(SeekFrom::Current(id_len as i64))
            .map_err(|_| GameError::Parse("Seek id"))?;
        let dl = c.read_u8().map_err(|_| GameError::Parse("Read desc len"))?;
        c.seek(SeekFrom::Current(dl as i64))
            .map_err(|_| GameError::Parse("Seek desc"))?;
        let tag_cnt = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read tag cnt"))?;
        for _ in 0..tag_cnt {
            let k = c.read_u8().map_err(|_| GameError::Parse("Read key len"))?;
            c.seek(SeekFrom::Current(k as i64))
                .map_err(|_| GameError::Parse("Seek key"))?;
            let v = c.read_u8().map_err(|_| GameError::Parse("Read val len"))?;
            c.seek(SeekFrom::Current(v as i64))
                .map_err(|_| GameError::Parse("Seek val"))?;
        }
        let ef = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read ext cnt"))?;
        c.seek(SeekFrom::Current((ef as i64) * 12))
            .map_err(|_| GameError::Parse("Seek ext"))?;
        let out_cnt = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read out cnt"))?;
        c.seek(SeekFrom::Current((out_cnt as i64) * 3))
            .map_err(|_| GameError::Parse("Seek outs"))?;
        let tr_cnt = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read trans cnt"))?;
        if tr_cnt == 0 {
            return Err(GameError::Parse("No translations"));
        }
        let lang_len = c.read_u8().map_err(|_| GameError::Parse("Read lang len"))?;
        c.seek(SeekFrom::Current(lang_len as i64))
            .map_err(|_| GameError::Parse("Seek lang"))?;
        let mut cid = [0; 3];
        c.read_exact(&mut cid)
            .map_err(|_| GameError::Parse("Read cid"))?;
        Ok(cid)
    }

    fn parse_node_edges_ids(data: &[u8]) -> Result<Vec<[u8; 3]>, GameError> {
        let mut c = Cursor::new(data);
        let id_len = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read id_len"))?;
        c.seek(SeekFrom::Current(id_len as i64))
            .map_err(|_| GameError::Parse("Seek id"))?;
        let dl = c.read_u8().map_err(|_| GameError::Parse("Read desc len"))?;
        c.seek(SeekFrom::Current(dl as i64))
            .map_err(|_| GameError::Parse("Seek desc"))?;
        let tag_cnt = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read tag cnt"))?;
        for _ in 0..tag_cnt {
            let k = c.read_u8().map_err(|_| GameError::Parse("Read key len"))?;
            c.seek(SeekFrom::Current(k as i64))
                .map_err(|_| GameError::Parse("Seek key"))?;
            let v = c.read_u8().map_err(|_| GameError::Parse("Read val len"))?;
            c.seek(SeekFrom::Current(v as i64))
                .map_err(|_| GameError::Parse("Seek val"))?;
        }
        let ef = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read ext cnt"))?;
        c.seek(SeekFrom::Current((ef as i64) * 12))
            .map_err(|_| GameError::Parse("Seek ext"))?;
        let out_cnt = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read out cnt"))?;
        let mut cids = Vec::with_capacity(out_cnt as usize);
        for _ in 0..out_cnt {
            let mut cid = [0; 3];
            c.read_exact(&mut cid)
                .map_err(|_| GameError::Parse("Read cid"))?;
            cids.push(cid);
        }
        Ok(cids)
    }

    fn parse_content_text(data: &[u8]) -> Result<String, GameError> {
        let mut c = Cursor::new(data);
        let id_len = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read id_len"))?;
        c.seek(SeekFrom::Current(id_len as i64))
            .map_err(|_| GameError::Parse("Seek id"))?;
        let txt_len = c
            .read_u32::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read txt_len"))? as usize;
        let mut buf = vec![0; txt_len];
        c.read_exact(&mut buf)
            .map_err(|_| GameError::Parse("Read text"))?;
        String::from_utf8(buf).map_err(|_| GameError::Parse("Invalid UTF-8"))
    }

    fn parse_edge_label_dest_cids(data: &[u8]) -> Result<([u8; 3], [u8; 3]), GameError> {
        let mut c = Cursor::new(data);
        let id_len = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read id_len"))?;
        c.seek(SeekFrom::Current(id_len as i64))
            .map_err(|_| GameError::Parse("Seek id"))?;
        c.seek(SeekFrom::Current(3))
            .map_err(|_| GameError::Parse("Seek from"))?;
        let mut dest_cid = [0; 3];
        c.read_exact(&mut dest_cid)
            .map_err(|_| GameError::Parse("Read dest"))?;
        let guard_cnt = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read guard cnt"))?;
        c.seek(SeekFrom::Current((guard_cnt as i64) * 12))
            .map_err(|_| GameError::Parse("Seek guard"))?;
        let label_cnt = c
            .read_u16::<LittleEndian>()
            .map_err(|_| GameError::Parse("Read label cnt"))?;
        if label_cnt == 0 {
            return Err(GameError::Parse("No edge labels"));
        }
        let lang_len = c.read_u8().map_err(|_| GameError::Parse("Read lang len"))?;
        c.seek(SeekFrom::Current(lang_len as i64))
            .map_err(|_| GameError::Parse("Seek lang"))?;
        let mut label_cid = [0; 3];
        c.read_exact(&mut label_cid)
            .map_err(|_| GameError::Parse("Read label"))?;
        Ok((label_cid, dest_cid))
    }

    async fn fetch_full(win: &Window, url: &str) -> Result<Vec<u8>, JsValue> {
        let resp = JsFuture::from(win.fetch_with_str(url))
            .await?
            .dyn_into::<Response>()?;
        if !resp.ok() {
            return Err(GameError::Http(resp.status()).into());
        }
        let buf = JsFuture::from(resp.array_buffer()?).await?;
        let arr = Uint8Array::new(&buf);
        let mut v = vec![0; arr.length() as usize];
        arr.copy_to(&mut v);
        Ok(v)
    }

    async fn fetch_ranges(
        &self,
        win: &Window,
        url: &str,
        ranges: &[(u64, u64)],
    ) -> Result<Vec<Vec<u8>>, JsValue> {
        let fetches = ranges
            .iter()
            .map(|&(s, e)| Self::fetch_range(win, url, s, Some(e)));
        let parts = try_join_all(fetches).await?;
        Ok(parts)
    }

    async fn get_raw_chunk(&self, entry: &IndexEntry) -> Result<Arc<Vec<u8>>, JsValue> {
        if let Some(cached) = self.raw_cache.borrow_mut().get(&entry.chunk_id) {
            return Ok(cached);
        }
        let win = window()
            .ok_or(GameError::Other("No window".to_string()))
            .map_err(JsValue::from)?;
        let data = if self.supports_range {
            Self::fetch_range(
                &win,
                &self.url,
                entry.offset,
                Some(entry.offset + entry.length as u64 - 1),
            )
            .await?
        } else {
            Self::fetch_full(&win, &self.url).await?
        };
        let arc = Arc::new(data);
        self.raw_cache
            .borrow_mut()
            .insert(entry.chunk_id, arc.clone());
        Ok(arc)
    }
}
