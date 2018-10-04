extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(s: &str) {
    syn::parse_str::<syn::Expr>(s).unwrap();
}
