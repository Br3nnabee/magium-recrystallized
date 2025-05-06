use std::collections::VecDeque;
use std::sync::Arc;
use std::cell::RefCell;
use wasm_bindgen::JsValue;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use web_sys::{window, Window};

use crate::types::IndexEntry;
use crate::http::{fetch_range, fetch_full};
use crate::utils::GameError;

/// Simple LRU cache of raw chunk blobs, keyed by ID.
pub struct RawCache {
    entries: VecDeque<([u8; 3], Arc<Vec<u8>>)>,
    capacity: usize,
}

impl RawCache {
    /// Create a new cache with the given capacity.
    pub fn new(cap: usize) -> Self {
        Self { entries: VecDeque::new(), capacity: cap }
    }

    /// Get a chunk by key, bumping it to the front if found.
    pub fn get(&mut self, key: &[u8; 3]) -> Option<Arc<Vec<u8>>> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
            let (k, v) = self.entries.remove(pos).unwrap();
            self.entries.push_front((k, v.clone()));
            return Some(v);
        }
        None
    }

    /// Insert a new chunk, evicting the oldest if full.
    pub fn insert(&mut self, key: [u8; 3], value: Arc<Vec<u8>>) {
        if self.entries.len() == self.capacity {
            self.entries.pop_back();
        }
        self.entries.push_front((key, value));
    }
}

/// Retrieves the raw chunk bytes for an entry, using HTTP Range requests
/// when supported, otherwise falling back to a full fetch. Uses an LRU cache.
pub async fn get_raw_chunk(
    cache: &RefCell<RawCache>,
    supports_range: bool,
    url: &str,
    entry: &IndexEntry,
) -> Result<Arc<Vec<u8>>, JsValue> {
    // Check cache first
    if let Some(cached) = cache.borrow_mut().get(&entry.chunk_id) {
        return Ok(cached);
    }

    // Acquire window
    let win: Window = window().ok_or(GameError::Other("No window".to_string())).map_err(JsValue::from)?;
    let data = if supports_range {
        // Range request
        fetch_range(&win, url, entry.offset, Some(entry.offset + entry.length as u64 - 1)).await?
    } else {
        // Full fetch
        fetch_full(&win, url).await?
    };

    let arc = Arc::new(data);
    cache.borrow_mut().insert(entry.chunk_id, arc.clone());
    Ok(arc)
}
