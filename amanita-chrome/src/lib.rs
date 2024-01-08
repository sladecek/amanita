mod utils;

use wasm_bindgen::prelude::*;
use gloo_console as console;
use rand_core::{RngCore, OsRng};
use amanita_lib::compact_test;


#[wasm_bindgen]
extern {
    fn alert (s: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn log (s: &str);
}


#[wasm_bindgen]
pub fn hello_content() {
   let result = compact_test(OsRng);
   console::info!("test finished {}", result);    
}

// Will be called in background.js
#[wasm_bindgen]
pub fn hello_background () {
    log("Hello from the background script!");
}
