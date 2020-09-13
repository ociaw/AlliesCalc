mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // Rust Analyzer reports this is unsafe, but also reports that the `unsafe`
    // block is unused, so we just surpress the warning.
    #[allow(unused_unsafe)]
    unsafe {
        alert("Hello, wasm!");
    }
}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}
