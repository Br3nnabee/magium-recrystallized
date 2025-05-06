use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use web_sys::{Headers, RequestInit, RequestMode, Response, Window};
use js_sys::Uint8Array;
use std::sync::Arc;
use futures::future::try_join_all;

use crate::utils::GameError;

/// Probes the remote file at `url` by requesting the first byte (bytes=0-0)
/// to determine total size and HTTP Range support.
pub async fn probe_range(win: &Window, url: &str) -> Result<(u64, bool), GameError> {
    let mut init = RequestInit::new();
    init.method("GET");
    init.mode(RequestMode::SameOrigin);
    let mut hdrs = Headers::new().map_err(|e| GameError::Other(format!("{:?}", e)))?;
    hdrs.append("Range", "bytes=0-0").map_err(|e| GameError::Other(format!("{:?}", e)))?;
    init.headers(&hdrs.into());
    let resp_val = JsFuture::from(win.fetch_with_str_and_init(url, &init))
        .await
        .map_err(|_| GameError::Http(0))?;
    let resp: Response = resp_val.dyn_into().map_err(|_| GameError::Other("Invalid response".to_string()))?;
    let status = resp.status();
    let ranged = status == 206;
    let size = if ranged {
        let cr = resp.headers().get("content-range").map_err(|_| GameError::Other("No Content-Range".to_string()))?.ok_or(GameError::Other("Bad Content-Range".to_string()))?;
        cr.split('/').nth(1).ok_or(GameError::Other("Bad Content-Range".to_string()))?.parse().map_err(|_| GameError::Other("Bad Content-Range".to_string()))?
    } else {
        let cl = resp.headers().get("content-length").map_err(|_| GameError::Other("No Content-Length".to_string()))?.ok_or(GameError::Other("Bad Content-Length".to_string()))?;
        cl.parse().map_err(|_| GameError::Other("Bad Content-Length".to_string()))?
    };
    Ok((size, ranged))
}

/// Fetches a specific byte range `[start..=end]` from the remote file.
pub async fn fetch_range(
    win: &Window,
    url: &str,
    start: u64,
    end: Option<u64>,
) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    let mut init = RequestInit::new();
    init.method("GET");
    init.mode(RequestMode::SameOrigin);
    let mut hdrs = Headers::new().map_err(|e| wasm_bindgen::JsValue::from_str(&format!("{:?}", e)))?;
    let range = match end { Some(e) => format!("bytes={}-{}", start, e), None => format!("bytes={}-", start) };
    hdrs.append("Range", &range).map_err(|e| wasm_bindgen::JsValue::from_str(&format!("{:?}", e)))?;
    init.headers(&hdrs.into());
    let resp = JsFuture::from(win.fetch_with_str_and_init(url, &init)).await?.dyn_into::<Response>()?;
    if !resp.ok() { return Err(wasm_bindgen::JsValue::from_str(&format!("HTTP error: {}", resp.status()))); }
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    let arr = Uint8Array::new(&buf);
    let mut v = vec![0; arr.length() as usize];
    arr.copy_to(&mut v);
    Ok(v)
}

/// Fetches the entire file at `url` without using Range requests.
pub async fn fetch_full(win: &Window, url: &str) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    let resp = JsFuture::from(win.fetch_with_str(url)).await?.dyn_into::<Response>()?;
    if !resp.ok() { return Err(wasm_bindgen::JsValue::from_str(&format!("HTTP error: {}", resp.status()))); }
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    let arr = Uint8Array::new(&buf);
    let mut v = vec![0; arr.length() as usize];
    arr.copy_to(&mut v);
    Ok(v)
}

/// Issues multiple `fetch_range` calls in parallel for each byte range.
pub async fn fetch_ranges(
    win: &Window,
    url: &str,
    ranges: &[(u64, u64)],
) -> Result<Vec<Vec<u8>>, wasm_bindgen::JsValue> {
    let parts = try_join_all(ranges.iter().map(|&(s, e)| fetch_range(win, url, s, Some(e)))).await?;
    Ok(parts)
}
