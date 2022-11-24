use wasm_bindgen::prelude::*;

#[link(wasm_import_module = "../shim.js")]
extern { fn alert2(); }

#[link(wasm_import_module = "../shim.js")]
extern { fn appendNumberToBody(n: i32); }

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn alert2_test() {
    unsafe {
        alert2();
    }
}

#[wasm_bindgen]
pub fn append_number_to_body_test(n: i32) {
    unsafe {
        appendNumberToBody(n);
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
