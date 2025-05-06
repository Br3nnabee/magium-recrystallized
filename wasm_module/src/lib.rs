//! # CYOA WASM Game Loader
//!
//! This crate provides the WebAssembly entry point and high-level
//! initialization for the CYOA game loader module. It wires up panic hooks,
//! an optional `wee_alloc` global allocator, and exposes the core `decoder`
//! functionality for use in JavaScript via `wasm_bindgen`.
//!
//! ## Features
//!
//! - **Optional Global Allocator**: Enable the `wee_alloc` feature for a
//!   minimal allocator suitable for size-constrained WASM builds.
//! - **Panic Hook**: Installs a panic hook that sends Rust panic messages
//!   to the browser console via `console.error`, simplifying debugging.
//! - **Auto-Initialization**: Uses `#[wasm_bindgen(start)]` to automatically
//!   initialize the module when loaded in JavaScript.

extern crate cfg_if;
extern crate wasm_bindgen;

/// Core decoding logic for the CYOA format.
///
/// The `decoder` module implements the `CyoaGame` struct and its associated
/// methods, including HTTP range probing, TLV parsing, zstd decompression,
/// and the public WASM-bindgen interface.
mod types;
mod tlv;
mod http;
mod cache;

mod game;

mod wasmtable;

/// Utility helpers and browser integration code.
///
/// The `utils` module provides support routines such as setting
/// up a panic hook for better error reporting in the browser.
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use utils::set_panic_hook;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        /// Use `wee_alloc` as the global allocator to minimize WASM binary size.
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

/// Entry point invoked by `wasm_bindgen` when the module is instantiated.
///
/// Installs the panic hook so that any Rust panics are forwarded to the
/// browser console as `console.error` messages, improving runtime
/// diagnostics when using the module from JavaScript.
#[wasm_bindgen(start)]
pub fn __wasm_start() {
    set_panic_hook();
}

