
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    log(format!("Adding {} and {}", a, b).as_str());
    return a + b;
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}.", name)
}

