use serde::Serialize;

/// All possible chunk types in the CYOA file format.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkType {
    /// A content node holding text and edge references.
    Node = 0x01,
    /// A binary blob encoding one edge’s metadata.
    Edge = 0x02,
    /// A text payload (e.g. node text or edge label).
    Content = 0x03,
    /// Metadata chunks (e.g. root-pointer).
    Metadata = 0x04,
    /// Pool of argument blobs (internal use).
    ArgBlobPool = 0xFD,
    /// WASM table data (internal use).
    WasmTable = 0xFE,
}

/// One entry in the on-disk index: type, ID, offset and length.
#[derive(Clone, Debug)]
pub struct IndexEntry {
    pub chunk_type: ChunkType,
    pub chunk_id: [u8; 3],
    pub offset: u64,
    pub length: u32,
}

/// If present, the guard consists of the function ID (u32)
/// and the raw guard‐bytes that follow in the TLV.
pub struct ContentEntry {
    pub guard: Option<(u32, Vec<u8>)>,
    pub content_id: [u8; 3],
}

/// Represents one outgoing edge from a node.
#[derive(Serialize)]
pub struct EdgeOutput {
    /// Text label shown for this choice.
    pub label: String,
    /// Index of the node this edge points to.
    pub dest_idx: u32,
}

/// The in‐memory representation of a game node:
/// its content text plus all outgoing edges.
#[derive(Serialize)]
pub struct NodeOutput {
    /// The narrative or choice text.
    pub content: String,
    /// All outgoing edges (choices).
    pub edges: Vec<EdgeOutput>,
}
