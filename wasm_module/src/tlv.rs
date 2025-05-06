use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::types::ChunkType;
use crate::types::IndexEntry;
use crate::utils::GameError;

/// Parses the fixed‐length file header and returns the byte offset
/// where the index blob begins.
pub fn parse_header(header: &[u8]) -> Result<u64, GameError> {
    let mut c = Cursor::new(header);
    let mut magic = [0; 4];
    c.read_exact(&mut magic).map_err(|_| GameError::InvalidMagic)?;
    if &magic != b"CYOA" {
        return Err(GameError::InvalidMagic);
    }
    c.seek(SeekFrom::Current(10)).map_err(|_| GameError::Parse("Seek error"))?;
    c.read_u64::<LittleEndian>().map_err(|_| GameError::Parse("Read u64 error"))
}

/// Parses the on‐disk index blob into a `Vec<IndexEntry>`.
pub fn parse_index(blob: &[u8]) -> Result<Vec<IndexEntry>, GameError> {
    let mut c = Cursor::new(blob);
    let cnt = c.read_u32::<LittleEndian>().map_err(|_| GameError::Parse("Read u32 error"))?;
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
        c.read_exact(&mut id).map_err(|_| GameError::Parse("Read id"))?;
        let off = c.read_u64::<LittleEndian>().map_err(|_| GameError::Parse("Read offset"))?;
        let len = c.read_u32::<LittleEndian>().map_err(|_| GameError::Parse("Read length"))?;
        out.push(IndexEntry { chunk_type, chunk_id: id, offset: off, length: len });
    }
    Ok(out)
}

/// Reads a TLV chunk header from `raw` and returns:
/// `(type, id, flags, compressed_len, uncompressed_len_opt, header_len)`.
pub fn parse_tlv_header(
    raw: &[u8],
) -> Result<(u8, [u8; 3], u8, u32, Option<u32>, usize), GameError> {
    let mut c = Cursor::new(raw);
    let t = c.read_u8().map_err(|_| GameError::Parse("Read type"))?;
    let mut id = [0; 3];
    c.read_exact(&mut id).map_err(|_| GameError::Parse("Read id"))?;
    let flags = c.read_u8().map_err(|_| GameError::Parse("Read flags"))?;
    let comp = c.read_u32::<LittleEndian>().map_err(|_| GameError::Parse("Read comp len"))?;
    let mut hlen = 1 + 3 + 1 + 4;
    let un = if flags & 1 != 0 {
        let u = c.read_u32::<LittleEndian>().map_err(|_| GameError::Parse("Read unlen"))?;
        hlen += 4;
        Some(u)
    } else {
        None
    };
    Ok((t, id, flags, comp, un, hlen))
}

/// Decompresses the given `data` slice with zstd if `flags & 1 != 0`,
/// otherwise returns `data` directly.
pub fn decompress_payload(
    flags: u8,
    data: &[u8],
    un: Option<u32>,
) -> Result<Vec<u8>, GameError> {
    if flags & 1 != 0 {
        let target = un.ok_or(GameError::Parse("Missing uncompressed length"))? as usize;
        let mut out = vec![0u8; target];
        let written = zstd_safe::decompress(out.as_mut_slice(), data)
            .map_err(|e| GameError::Other(e.to_string()))?;
        out.truncate(written);
        Ok(out)
    } else {
        Ok(data.to_vec())
    }
}

/// Reads a UTF-8 text string from a `ChunkType::Content` payload.
pub fn parse_content_text(data: &[u8]) -> Result<String, GameError> {
    let mut c = Cursor::new(data);
    let id_len = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Read id_len"))?;
    c.seek(SeekFrom::Current(id_len as i64)).map_err(|_| GameError::Parse("Seek id"))?;
    let txt_len = c.read_u32::<LittleEndian>().map_err(|_| GameError::Parse("Read txt_len"))? as usize;
    let mut buf = vec![0; txt_len];
    c.read_exact(&mut buf).map_err(|_| GameError::Parse("Read text"))?;
    String::from_utf8(buf).map_err(|_| GameError::Parse("Invalid UTF-8"))
}

/// Extracts all outgoing edge‐CIDs from a node’s payload.
pub fn parse_node_edges_ids(data: &[u8]) -> Result<Vec<[u8; 3]>, GameError> {
    let mut c = Cursor::new(data);
    // Skip Node ID
    let id_len = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Read ID length"))?;
    c.seek(SeekFrom::Current(id_len as i64)).map_err(|_| GameError::Parse("Skip ID"))?;
    // Skip default language
    let dl = c.read_u8().map_err(|_| GameError::Parse("Read default language length"))?;
    c.seek(SeekFrom::Current(dl as i64)).map_err(|_| GameError::Parse("Skip default language"))?;
    // Skip tags
    let tag_cnt = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Read tag count"))?;
    for _ in 0..tag_cnt {
        let k = c.read_u8().map_err(|_| GameError::Parse("Read tag key length"))?;
        c.seek(SeekFrom::Current(k as i64)).map_err(|_| GameError::Parse("Skip tag key"))?;
        let v = c.read_u8().map_err(|_| GameError::Parse("Read tag value length"))?;
        c.seek(SeekFrom::Current(v as i64)).map_err(|_| GameError::Parse("Skip tag value"))?;
    }
    // Skip entry functions
    let ef = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Read entry_funcs count"))?;
    c.seek(SeekFrom::Current((ef as i64) * 12)).map_err(|_| GameError::Parse("Skip entry_funcs"))?;
    // Read outgoing edges
    let out_cnt = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Read outgoing count"))?;
    let mut ids = Vec::with_capacity(out_cnt as usize);
    for _ in 0..out_cnt {
        let mut cid = [0u8; 3];
        c.read_exact(&mut cid).map_err(|_| GameError::Parse("Read outgoing CID"))?;
        ids.push(cid);
    }
    Ok(ids)
}

/// Parses an edge’s metadata payload, returning `(label_cid, dest_cid)`.
pub fn parse_edge_label_dest_cids(data: &[u8]) -> Result<([u8; 3], [u8; 3]), GameError> {
    let mut c = Cursor::new(data);
    let id_len = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Read id_len"))?;
    c.seek(SeekFrom::Current(id_len as i64)).map_err(|_| GameError::Parse("Seek id"))?;
    c.seek(SeekFrom::Current(3)).map_err(|_| GameError::Parse("Seek from"))?;
    let mut dest_cid = [0; 3];
    c.read_exact(&mut dest_cid).map_err(|_| GameError::Parse("Read dest"))?;
    let guard_cnt = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Read guard cnt"))?;
    c.seek(SeekFrom::Current((guard_cnt as i64) * 12)).map_err(|_| GameError::Parse("Seek guard"))?;
    let label_cnt = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Read label cnt"))?;
    if label_cnt == 0 {
        return Err(GameError::Parse("No edge labels"));
    }
    let lang_len = c.read_u8().map_err(|_| GameError::Parse("Read lang len"))?;
    c.seek(SeekFrom::Current(lang_len as i64)).map_err(|_| GameError::Parse("Seek lang"))?;
    let mut label_cid = [0; 3];
    c.read_exact(&mut label_cid).map_err(|_| GameError::Parse("Read label"))?;
    Ok((label_cid, dest_cid))
}

/// Parse the content_sequence entries from a Node payload slice.
pub fn parse_node_content_seq(data: &[u8]) -> Result<Vec<crate::types::ContentEntry>, GameError> {
    let mut c = Cursor::new(data);
    skip_node_header_fields(&mut c)?;
    let seq_cnt = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Read content_seq count"))?;
    let mut out = Vec::with_capacity(seq_cnt as usize);
    for _ in 0..seq_cnt {
        let has = c.read_u8().map_err(|_| GameError::Parse("Read has_guard"))? != 0;
        let guard = if has {
            let func_id = c.read_u32::<LittleEndian>().map_err(|_| GameError::Parse("Read func_id"))?;
            let off = c.read_u32::<LittleEndian>().map_err(|_| GameError::Parse("Read arg_off"))? as usize;
            let len = c.read_u32::<LittleEndian>().map_err(|_| GameError::Parse("Read arg_len"))? as usize;
            let guard_bytes = data.get(off..off+len).ok_or(GameError::Parse("Guard slice out of range"))?.to_vec();
            Some((func_id, guard_bytes))
        } else {
            c.seek(SeekFrom::Current(12)).map_err(|_| GameError::Parse("Skip dummy guard fields"))?;
            None
        };
        let mut cid = [0u8; 3];
        c.read_exact(&mut cid).map_err(|_| GameError::Parse("Read content_cid"))?;
        out.push(crate::types::ContentEntry { guard, content_id: cid });
    }
    Ok(out)
}

/// Skip over common Node payload fields up to the content_sequence.
pub fn skip_node_header_fields<R: Read + Seek>(c: &mut R) -> Result<(), GameError> {
    let id_len = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Skip ID length"))?;
    c.seek(SeekFrom::Current(id_len as i64)).map_err(|_| GameError::Parse("Skip ID"))?;
    let dl = c.read_u8().map_err(|_| GameError::Parse("Skip default_language length"))?;
    c.seek(SeekFrom::Current(dl as i64)).map_err(|_| GameError::Parse("Skip default_language"))?;
    let tag_cnt = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Skip tag count"))?;
    for _ in 0..tag_cnt {
        let k = c.read_u8().map_err(|_| GameError::Parse("Skip tag key length"))?;
        c.seek(SeekFrom::Current(k as i64)).map_err(|_| GameError::Parse("Skip tag key"))?;
        let v = c.read_u8().map_err(|_| GameError::Parse("Skip tag value length"))?;
        c.seek(SeekFrom::Current(v as i64)).map_err(|_| GameError::Parse("Skip tag value"))?;
    }
    let ef = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Skip entry_funcs count"))?;
    c.seek(SeekFrom::Current((ef as i64) * 12)).map_err(|_| GameError::Parse("Skip entry_funcs"))?;
    let out_cnt = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Skip outgoing count"))?;
    c.seek(SeekFrom::Current((out_cnt as i64)*3)).map_err(|_| GameError::Parse("Skip outgoing IDs"))?;
    let tr_cnt = c.read_u16::<LittleEndian>().map_err(|_| GameError::Parse("Skip translations count"))?;
    for _ in 0..tr_cnt {
        let ll = c.read_u8().map_err(|_| GameError::Parse("Skip translation lang length"))?;
        c.seek(SeekFrom::Current(ll as i64)).map_err(|_| GameError::Parse("Skip translation lang"))?;
        c.seek(SeekFrom::Current(3)).map_err(|_| GameError::Parse("Skip translation CID"))?;
    }
    Ok(())
}
