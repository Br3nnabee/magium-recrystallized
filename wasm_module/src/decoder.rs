use byteorder::{LittleEndian, ReadBytesExt};
use js_sys::{Array, Uint8Array};
use std::io::{Cursor, Read, Seek, SeekFrom};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, RequestInit, RequestMode, Response, Window, window};
use zstd_safe::decompress;

const HEADER_LEN: usize = 22;
const CHUNK_NODE: u8 = 0x01;
const CHUNK_EDGE: u8 = 0x02;
const CHUNK_CONTENT: u8 = 0x03;
const CHUNK_METADATA: u8 = 0x04;
const ID_ROOT_POINTER: [u8; 3] = [0, 0, 1];

#[derive(Clone, Debug)]
struct IndexEntry {
    chunk_type: u8,
    chunk_id: [u8; 3],
    offset: u64,
    length: u32,
}

#[wasm_bindgen]
pub struct CyoaGame {
    url: String,
    size: u64,
    supports_range: bool,
    index: Vec<IndexEntry>,
}

#[wasm_bindgen]
impl CyoaGame {
    /// Async constructor
    #[wasm_bindgen(constructor)]
    pub async fn new(path: String) -> Result<CyoaGame, JsValue> {
        let url = if path.starts_with('/') {
            path.clone()
        } else {
            format!("/{}", path)
        };
        let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
        let (size, supports_range) = Self::probe_range(&win, &url).await?;
        let header = if supports_range {
            Self::fetch_range(&win, &url, 0, Some((HEADER_LEN - 1) as u64)).await?
        } else {
            Self::fetch_full(&win, &url).await?
        };
        let index_offset = Self::parse_header(&header)?;
        if index_offset >= size {
            return Err(JsValue::from_str("index out of range"));
        }
        let idx_blob = if supports_range {
            Self::fetch_range(&win, &url, index_offset, None).await?
        } else {
            header[index_offset as usize..].to_vec()
        };
        let index = Self::parse_index(&idx_blob)?;
        Ok(CyoaGame {
            url,
            size,
            supports_range,
            index,
        })
    }

    /// Return all chunk IDs as hex strings
    #[wasm_bindgen]
    pub fn chunk_ids(&self) -> Array {
        self.index
            .iter()
            .map(|e| {
                let s = e
                    .chunk_id
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<String>();
                JsValue::from_str(&s)
            })
            .collect()
    }

    /// Async: get root node chunk ID hex
    #[wasm_bindgen]
    pub async fn get_root_node(&self) -> Result<String, JsValue> {
        let entry = self
            .index
            .iter()
            .find(|e| e.chunk_type == CHUNK_METADATA && e.chunk_id == ID_ROOT_POINTER)
            .ok_or_else(|| JsValue::from_str("no root-pointer metadata"))?;
        let raw = self.get_raw_chunk(entry).await?;
        let (_ct, _id, flags, comp_len, unlen, hdr_len) = Self::parse_tlv_header(&raw)?;
        let payload =
            Self::decompress_payload(flags, &raw[hdr_len..hdr_len + comp_len as usize], unlen)?;
        let hex = payload[0..3]
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>();
        Ok(hex)
    }

    /// Async: fetch content chunk text
    #[wasm_bindgen]
    pub async fn get_content(&self, idx: usize) -> Result<String, JsValue> {
        let entry = self
            .index
            .get(idx)
            .ok_or_else(|| JsValue::from_str("index out of range"))?;
        let raw = self.get_raw_chunk(entry).await?;
        let (_ct, _id, flags, comp_len, unlen, hdr_len) = Self::parse_tlv_header(&raw)?;
        let data =
            Self::decompress_payload(flags, &raw[hdr_len..hdr_len + comp_len as usize], unlen)?;
        let mut cursor = Cursor::new(&data);
        let id_len = cursor
            .read_u16::<LittleEndian>()
            .map_err(|e| JsValue::from_str(&e.to_string()))? as u64;
        cursor
            .seek(SeekFrom::Current(id_len as i64))
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let txt_len = cursor
            .read_u32::<LittleEndian>()
            .map_err(|e| JsValue::from_str(&e.to_string()))? as usize;
        let mut buf = vec![0u8; txt_len];
        cursor
            .read_exact(&mut buf)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        String::from_utf8(buf).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Async: fetch node->content translation ID
    #[wasm_bindgen]
    pub async fn get_node_content(&self, idx: usize) -> Result<String, JsValue> {
        let entry = self
            .index
            .get(idx)
            .ok_or_else(|| JsValue::from_str("index out of range"))?;
        let raw = self.get_raw_chunk(entry).await?;
        let (_ct, _id, flags, comp_len, unlen, hdr_len) = Self::parse_tlv_header(&raw)?;
        let data =
            Self::decompress_payload(flags, &raw[hdr_len..hdr_len + comp_len as usize], unlen)?;
        let mut cursor = Cursor::new(&data);
        // skip to translations
        let id_len = cursor.read_u16::<LittleEndian>().unwrap() as u64;
        cursor.seek(SeekFrom::Current(id_len as i64)).unwrap();
        let dl = cursor.read_u8().unwrap() as i64;
        cursor.seek(SeekFrom::Current(dl)).unwrap();
        let tag_cnt = cursor.read_u16::<LittleEndian>().unwrap();
        for _ in 0..tag_cnt {
            let k = cursor.read_u8().unwrap() as i64;
            cursor.seek(SeekFrom::Current(k)).unwrap();
            let v = cursor.read_u8().unwrap() as i64;
            cursor.seek(SeekFrom::Current(v)).unwrap();
        }
        let ef = cursor.read_u16::<LittleEndian>().unwrap();
        cursor.seek(SeekFrom::Current((ef as i64) * 12)).unwrap();
        let out_cnt = cursor.read_u16::<LittleEndian>().unwrap() as i64;
        cursor.seek(SeekFrom::Current(out_cnt * 3)).unwrap();
        let tr_cnt = cursor.read_u16::<LittleEndian>().unwrap();
        if tr_cnt == 0 {
            return Err(JsValue::from_str("no translations"));
        }
        let lang_len = cursor.read_u8().unwrap() as i64;
        cursor.seek(SeekFrom::Current(lang_len)).unwrap();
        let mut cid = [0u8; 3];
        cursor.read_exact(&mut cid).unwrap();
        let hex = cid.iter().map(|b| format!("{:02X}", b)).collect::<String>();
        Ok(hex)
    }

    #[wasm_bindgen]
    pub async fn get_edges(&self, idx: usize) -> Result<Array, JsValue> {
        // locate and fetch the raw node chunk
        let entry = self
            .index
            .get(idx)
            .ok_or_else(|| JsValue::from_str("index out of range"))?;
        let raw = self.get_raw_chunk(entry).await?;

        // parse TLV header
        let (_ct, _id, flags, comp_len, unlen, hdr_len) = Self::parse_tlv_header(&raw)?;

        // decompress or copy payload
        let data = match Self::decompress_payload(
            flags,
            &raw[hdr_len..hdr_len + comp_len as usize],
            unlen,
        ) {
            Ok(d) => d,
            Err(e) => {
                web_sys::console::error_1(&e);
                return Ok(Array::new());
            }
        };
        let mut cursor = Cursor::new(&data);

        // helper to convert I/O errors into JsValue
        let io_to_js = |e: std::io::Error| JsValue::from_str(&e.to_string());

        // 1) skip node ID (u16 length + bytes)
        let id_len = cursor.read_u16::<LittleEndian>().map_err(io_to_js)? as u64;
        cursor.set_position(cursor.position() + id_len);

        // 2) skip default_language (u8 length + bytes)
        let dl = cursor.read_u8().map_err(io_to_js)? as u64;
        cursor.set_position(cursor.position() + dl);

        // 3) skip tags (u16 count + each key/value pair)
        let tag_cnt = cursor.read_u16::<LittleEndian>().map_err(io_to_js)?;
        for _ in 0..tag_cnt {
            let k = cursor.read_u8().map_err(io_to_js)? as u64;
            cursor.set_position(cursor.position() + k);
            let v = cursor.read_u8().map_err(io_to_js)? as u64;
            cursor.set_position(cursor.position() + v);
        }

        // 4) skip entry_funcs (u16 count + 12 bytes each)
        let ef_cnt = cursor.read_u16::<LittleEndian>().map_err(io_to_js)? as u64;
        cursor.set_position(cursor.position() + ef_cnt * 12);

        // 5) read outgoing edge count
        let out_cnt = cursor.read_u16::<LittleEndian>().map_err(io_to_js)? as usize;
        let mut arr = Array::new();

        // 6) read each 3-byte edge ID and push hex strings
        for _ in 0..out_cnt {
            let mut eid = [0u8; 3];
            cursor
                .read_exact(&mut eid)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let hex = eid.iter().map(|b| format!("{:02X}", b)).collect::<String>();
            arr.push(&JsValue::from_str(&hex));
        }

        Ok(arr)
    }

    #[wasm_bindgen]
    pub async fn get_edge_label(&self, idx: usize) -> Result<String, JsValue> {
        // 1) pull down the raw edge chunk
        let entry = self
            .index
            .get(idx)
            .ok_or_else(|| JsValue::from_str("edge index out of range"))?;
        if entry.chunk_type != CHUNK_EDGE {
            return Err(JsValue::from_str("not an edge chunk"));
        }
        let raw = self.get_raw_chunk(entry).await?;

        // 2) strip off the TLV header
        let (_ct, _id, flags, comp_len, unlen, hdr_len) = Self::parse_tlv_header(&raw)?;
        let data =
            Self::decompress_payload(flags, &raw[hdr_len..hdr_len + comp_len as usize], unlen)?;

        // 3) walk the edge‐chunk fields until we reach the label table
        let mut cursor = Cursor::new(&data);
        // – skip edge ID (u16 + bytes)
        let id_len = cursor.read_u16::<LittleEndian>().unwrap() as u64;
        cursor.seek(SeekFrom::Current(id_len as i64)).unwrap();
        // – skip `from` + `to` (3 + 3 bytes)
        cursor.seek(SeekFrom::Current(6)).unwrap();
        // – skip guard_funcs (u16 count + each 4(fid)+4(off)+4(len) bytes)
        let gf_cnt = cursor.read_u16::<LittleEndian>().unwrap() as u64;
        cursor
            .seek(SeekFrom::Current((gf_cnt * 12) as i64))
            .unwrap();

        // – now read the labels array
        let lbl_cnt = cursor.read_u16::<LittleEndian>().unwrap();
        if lbl_cnt == 0 {
            return Err(JsValue::from_str("edge has no labels"));
        }
        // first label:
        let lang_len = cursor.read_u8().unwrap() as usize;
        cursor.seek(SeekFrom::Current(lang_len as i64)).unwrap();
        let mut cid = [0u8; 3];
        cursor.read_exact(&mut cid).unwrap();

        // 4) look up that content‐chunk in the index
        let content_entry = self
            .index
            .iter()
            .find(|e| e.chunk_type == CHUNK_CONTENT && e.chunk_id == cid)
            .ok_or_else(|| JsValue::from_str("content chunk not found"))?;

        // 5) fetch + decode the content chunk just like get_content()
        let raw_c = self.get_raw_chunk(content_entry).await?;
        let (_ct2, _id2, flags2, comp2, un2, hdr2) = Self::parse_tlv_header(&raw_c)?;
        let payload = Self::decompress_payload(flags2, &raw_c[hdr2..hdr2 + comp2 as usize], un2)?;

        // parse out the text after the content‐ID
        let mut c2 = Cursor::new(&payload);
        let idlen2 = c2.read_u16::<LittleEndian>().unwrap() as u64;
        c2.seek(SeekFrom::Current(idlen2 as i64)).unwrap();
        let txt_len = c2.read_u32::<LittleEndian>().unwrap() as usize;
        let mut buf = vec![0; txt_len];
        c2.read_exact(&mut buf).unwrap();

        String::from_utf8(buf).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Fetch and return the hex ID of the node this edge points to
    #[wasm_bindgen]
    pub async fn get_edge_destination(&self, idx: usize) -> Result<String, JsValue> {
        // 1) fetch raw edge chunk
        let entry = self
            .index
            .get(idx)
            .ok_or_else(|| JsValue::from_str("edge index out of range"))?;
        if entry.chunk_type != CHUNK_EDGE {
            return Err(JsValue::from_str("not an edge chunk"));
        }
        let raw = self.get_raw_chunk(entry).await?;

        // 2) strip TLV header and decompress
        let (_ct, _id, flags, comp_len, unlen, hdr_len) = Self::parse_tlv_header(&raw)?;
        let data =
            Self::decompress_payload(flags, &raw[hdr_len..hdr_len + comp_len as usize], unlen)?;

        // 3) walk to the `to` field
        let mut cursor = Cursor::new(&data);
        // skip ID
        let id_len = cursor.read_u16::<LittleEndian>().unwrap() as u64;
        cursor.seek(SeekFrom::Current(id_len as i64)).unwrap();
        // read & skip `from`
        cursor.seek(SeekFrom::Current(3)).unwrap();
        // read `to`
        let mut to_id = [0u8; 3];
        cursor.read_exact(&mut to_id).unwrap();

        // return hex string
        let hex = to_id
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>();
        Ok(hex)
    }

    // -- private helpers --
    async fn probe_range(win: &Window, url: &str) -> Result<(u64, bool), JsValue> {
        let mut init = RequestInit::new();
        init.set_method("GET");
        init.set_mode(RequestMode::SameOrigin);
        let mut hdrs = Headers::new()?;
        hdrs.append("Range", "bytes=0-0")?;
        init.set_headers(&hdrs.into());
        let resp = JsFuture::from(win.fetch_with_str_and_init(url, &init))
            .await?
            .dyn_into::<Response>()?;
        let ranged = resp.status() == 206;
        let size = if ranged {
            resp.headers()
                .get("content-range")?
                .ok_or(JsValue::from_str("no Content-Range"))?
                .split('/')
                .nth(1)
                .ok_or(JsValue::from_str("bad Content-Range"))?
                .parse::<u64>()
                .map_err(|e| JsValue::from_str(&e.to_string()))?
        } else {
            resp.headers()
                .get("content-length")?
                .ok_or(JsValue::from_str("no Content-Length"))?
                .parse::<u64>()
                .map_err(|e| JsValue::from_str(&e.to_string()))?
        };
        Ok((size, ranged))
    }

    async fn get_raw_chunk(&self, entry: &IndexEntry) -> Result<Vec<u8>, JsValue> {
        let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
        if self.supports_range {
            Self::fetch_range(
                &win,
                &self.url,
                entry.offset,
                Some(entry.offset + entry.length as u64 - 1),
            )
            .await
        } else {
            Self::fetch_full(&win, &self.url).await
        }
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
        let mut hdrs = Headers::new()?;
        let range = end.map_or_else(
            || format!("bytes={}-", start),
            |e| format!("bytes={}-{}", start, e),
        );
        hdrs.append("Range", &range)?;
        init.set_headers(&hdrs.into());
        let resp = JsFuture::from(win.fetch_with_str_and_init(url, &init))
            .await?
            .dyn_into::<Response>()?;
        if !resp.ok() {
            return Err(JsValue::from_str("HTTP error"));
        }
        let buf = JsFuture::from(resp.array_buffer()?).await?;
        let arr = Uint8Array::new(&buf);
        let mut v = vec![0u8; arr.length() as usize];
        arr.copy_to(&mut v);
        Ok(v)
    }

    async fn fetch_full(win: &Window, url: &str) -> Result<Vec<u8>, JsValue> {
        let resp = JsFuture::from(win.fetch_with_str(url))
            .await?
            .dyn_into::<Response>()?;
        if !resp.ok() {
            return Err(JsValue::from_str("HTTP error"));
        }
        let buf = JsFuture::from(resp.array_buffer()?).await?;
        let arr = Uint8Array::new(&buf);
        let mut v = vec![0u8; arr.length() as usize];
        arr.copy_to(&mut v);
        Ok(v)
    }

    fn parse_header(header: &[u8]) -> Result<u64, JsValue> {
        let mut c = Cursor::new(header);
        let mut magic = [0u8; 4];
        c.read_exact(&mut magic)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        if &magic != b"CYOA" {
            return Err(JsValue::from_str("bad magic"));
        }
        c.seek(SeekFrom::Current(10))
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        c.read_u64::<LittleEndian>()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    fn parse_index(blob: &[u8]) -> Result<Vec<IndexEntry>, JsValue> {
        let mut c = Cursor::new(blob);
        let cnt = c
            .read_u32::<LittleEndian>()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let mut v = Vec::with_capacity(cnt as usize);
        for _ in 0..cnt {
            let t = c.read_u8().unwrap();
            let mut id = [0u8; 3];
            c.read_exact(&mut id).unwrap();
            let off = c.read_u64::<LittleEndian>().unwrap();
            let len = c.read_u32::<LittleEndian>().unwrap();
            v.push(IndexEntry {
                chunk_type: t,
                chunk_id: id,
                offset: off,
                length: len,
            });
        }
        Ok(v)
    }

    fn parse_tlv_header(raw: &[u8]) -> Result<(u8, [u8; 3], u8, u32, Option<u32>, usize), JsValue> {
        let mut c = Cursor::new(raw);
        let t = c.read_u8().unwrap();
        let mut id = [0u8; 3];
        c.read_exact(&mut id).unwrap();
        let flags = c.read_u8().unwrap();
        let comp = c.read_u32::<LittleEndian>().unwrap();
        let mut hlen = 1 + 3 + 1 + 4;
        let un = if flags & 1 != 0 {
            let u = c.read_u32::<LittleEndian>().unwrap();
            hlen += 4;
            Some(u)
        } else {
            None
        };
        Ok((t, id, flags, comp, un, hlen))
    }

    fn decompress_payload(flags: u8, data: &[u8], un: Option<u32>) -> Result<Vec<u8>, JsValue> {
        if flags & 1 != 0 {
            let target =
                un.ok_or_else(|| JsValue::from_str("missing uncompressed length"))? as usize;
            let mut out = vec![0u8; target];
            let written =
                decompress(&mut out[..], data).map_err(|e| JsValue::from_str(&e.to_string()))?;
            out.truncate(written);
            Ok(out)
        } else {
            Ok(data.to_vec())
        }
    }
}
