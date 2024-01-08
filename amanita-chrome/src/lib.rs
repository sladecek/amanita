mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature="wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert (s: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn log (s: &str);
}

// Will be called in content.js
#[wasm_bindgen]
pub fn hello_content () {
    alert("Hello from the content script!");
}

// Will be called in background.js
#[wasm_bindgen]
pub fn hello_background () {
    log("Hello from the background script!");
}
