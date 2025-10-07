// Simple WASM test to verify bindings work
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmTest {
    value: u32,
}

#[wasm_bindgen]
impl WasmTest {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { value: 42 }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u32 {
        self.value
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }
}

#[wasm_bindgen]
pub fn test_function() -> u32 {
    123
}