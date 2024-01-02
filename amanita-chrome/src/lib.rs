use gloo_console as console;
use gloo_utils::{body, document};
use wasm_bindgen::{prelude::*};

#[wasm_bindgen]
pub fn start() {
    console::info!("Start foreground script");
}

