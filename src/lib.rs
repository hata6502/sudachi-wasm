//! Clone of [Sudachi](https://github.com/WorksApplications/Sudachi),
//! a Japanese morphological analyzer
//!
//! The main entry point of the library is the
//! [`Tokenizer`](tokenizer/struct.Tokenizer.html) struct, which
//! implements [`Tokenize`](tokenizer/trait.Tokenize.html).

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate nom;

pub mod dic;
pub mod error;
pub mod lattice;
pub mod morpheme;
pub mod plugin;
pub mod tokenizer;
pub mod utf8inputtext;

pub use error::*;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{
        morpheme::Morpheme,
        tokenizer::{dictionary_bytes_from_path, Mode, Tokenize, Tokenizer},
        SudachiError, SudachiResult,
    };
}

extern crate wasm_bindgen;

use serde::Serialize;
use tokenizer::Mode;
use tokenizer::{Tokenize, Tokenizer};
use wasm_bindgen::prelude::*;

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
    let tokenizer = Tokenizer::from_dictionary_bytes(BYTES).unwrap();

    let morphemes = tokenizer
        .tokenize(
            &input,
            match mode {
                TokenizeMode::A => Mode::A,
                TokenizeMode::B => Mode::B,
                TokenizeMode::C => Mode::C,
            },
            false,
        )
        .unwrap();

    let mut described_morphemes = Vec::new();

    for morpheme in morphemes {
        described_morphemes.push(DescribedMorpheme {
            surface: morpheme.surface().clone(),
            poses: morpheme.pos().unwrap().clone(),
            normalized_form: morpheme.normalized_form().clone(),
            reading_form: morpheme.dictionary_form().clone(),
            dictionary_form: morpheme.reading_form().clone(),
        });
    }

    serde_json::to_string(&described_morphemes).unwrap()
}
