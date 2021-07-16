#[macro_use]
extern crate nom;

pub mod morpheme;
pub mod tokenizer;

pub mod dic;
pub mod lattice;

extern crate wasm_bindgen;

use serde::Serialize;
use wasm_bindgen::prelude::*;
use tokenizer::Mode;
use tokenizer::Tokenizer;

// embed dictionary binary file
const BYTES: &[u8; 122037852] = include_bytes!("resources/system.dic");

#[wasm_bindgen]
pub enum TokenizeMode {
    A,
    B,
    C,
}

#[derive(Serialize)]
struct DescribedMorpheme {
    surface: String,
    poses: Vec<String>,
    normalized_form: String,
    reading_form: String,
    dictionary_form: String,
}

#[wasm_bindgen]
pub fn tokenize(input: String, mode: TokenizeMode) -> String {
    // load and parse dictionary binary to create a tokenizer
    let tokenizer = Tokenizer::new(BYTES);

    let morphemes = tokenizer.tokenize(&input, match mode {
        TokenizeMode::A => &Mode::A,
        TokenizeMode::B => &Mode::B,
        TokenizeMode::C => &Mode::C,
    }, false);

    let mut described_morphemes = Vec::new();

    for morpheme in morphemes {
        described_morphemes.push(DescribedMorpheme {
            surface: morpheme.surface().clone(),
            poses: morpheme.pos().clone(),
            normalized_form: morpheme.normalized_form().clone(),
            reading_form: morpheme.dictionary_form().clone(),
            dictionary_form: morpheme.reading_form().clone(),
        });
    };

    serde_json::to_string(&described_morphemes).unwrap()
}
