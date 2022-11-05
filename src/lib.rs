use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn great(name: &str) {
    println!("Hi there {}", name);
}
