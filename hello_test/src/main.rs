
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    // js 函数
    fn alert(msg: &str);
}

#[wasm_bindgen]
pub fn alert_by_rust(msg: &str) {
    return alert(msg);
}


fn main() {
    println!("Hello, world!");
}