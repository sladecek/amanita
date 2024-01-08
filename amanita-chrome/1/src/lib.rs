use gloo_console as console;
use gloo_utils::{body, document};
use wasm_bindgen::{prelude::*};
use rand_core::{RngCore, OsRng};
use amanita_lib::compact_test;


#[wasm_bindgen]
pub fn start() {
    console::info!("Start foreground script");
    console::info!("test finished {}", compact_test(OsRng));
    
}

