use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SimpleTest {
    value: i32,
}

#[wasm_bindgen]
impl SimpleTest {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { value: 42 }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }
}