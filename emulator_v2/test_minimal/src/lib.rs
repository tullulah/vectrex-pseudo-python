use wasm_bindgen::prelude::*;

// Import the `console.log` function from the browser's console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to provide `println!(..)` style syntax for `console.log` logging.
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct TestClass {
    value: i32,
}

#[wasm_bindgen]
impl TestClass {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TestClass {
        console_log!("TestClass::new() called");
        TestClass { value: 42 }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    console_log!("Hello, {}!", name);
}