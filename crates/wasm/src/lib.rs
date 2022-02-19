use wasm_bindgen::prelude::*;
use mizl_parser;
use serde;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    let (_str, color) = mizl_parser::hex_color(input).unwrap();
    JsValue::from_serde(&color).unwrap()
}

#[wasm_bindgen]
pub fn setup() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    #[wasm_bindgen_test]
    fn pass() {
        let (_str, color) = mizl_parser::hex_color("#ff0000").unwrap();
        assert_eq!(color.r, 255);
    }
}
