mod util;
use wasm_bindgen::prelude::*;
use web_sys::console;
// use js_sys::Array;

// JsValue:
// https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    util::set_panic_hook();
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
