use wasm_bindgen::JsValue;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Errors that can occur while probing, fetching,
/// or parsing the CYOA file.
#[derive(Debug)]
pub enum GameError {
    /// Non-200 HTTP response, with status code.
    Http(u16),
    /// Server does not support HTTP range requests.
    RangeNotSupported,
    /// File magic header did not match CYOA.
    InvalidMagic,
    /// Index pointer points past end of file.
    IndexOutOfRange,
    /// A required TLV tag or translation was missing.
    Parse(&'static str),
    /// Root pointer metadata chunk was not found.
    MissingRoot,
    /// Other errors, with textual detail.
    Other(String),
}

impl From<GameError> for JsValue {
    fn from(err: GameError) -> JsValue {
        // Maps each GameError variant to a JS exception string.
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

/// Logs debug messages to the browser console when
/// compiled with debug_assertions.
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        if cfg!(debug_assertions) {
            web_sys::console::log_1(&JsValue::from_str(&format!($($arg)*)));
        }
    }};
}
