extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn sayHello(name: &str) {
    alert(&format!("Hello {}", name));
}
