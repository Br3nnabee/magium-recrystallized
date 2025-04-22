extern crate cfg_if;
extern crate wasm_bindgen;

mod decoder;
mod utils;

use cfg_if::cfg_if;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen(start)]
pub fn __wasm_start() {
    set_panic_hook();
}
