#[macro_use]
extern crate nom;

pub mod morpheme;
pub mod tokenizer;

pub mod dic;
pub mod lattice;

extern crate wasm_bindgen;

use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;
use tokenizer::Mode;
use tokenizer::Tokenizer;

// embed dictionary binary file
const BYTES: &[u8; 212973212] = include_bytes!("resources/system.dic");
// load and parse dictionary binary to create a tokenizer
const TOKENIZER: Lazy<Tokenizer> = Lazy::new(|| Tokenizer::new(BYTES));

#[wasm_bindgen]
pub fn tokenize() -> String {
    let input = String::from("すもももももももものうち");

    TOKENIZER.tokenize(&input, &Mode::C, false)[0].surface().clone()
}
