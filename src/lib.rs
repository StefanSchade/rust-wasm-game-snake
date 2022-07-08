use wasm_bindgen::prelude::*;

#[wasm-bindgen]
pub fn greet(name: &str) {
    println!("Hi there {}", name);
}

// wasm-pack build --target web