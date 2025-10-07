// Simplified WASM API to isolate the issue
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct VectrexEmulatorSimple {
    value: u32,
}

#[wasm_bindgen]
impl VectrexEmulatorSimple {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { value: 42 }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u32 {
        self.value
    }
}

#[wasm_bindgen]
pub fn simple_function() -> u32 {
    123
}