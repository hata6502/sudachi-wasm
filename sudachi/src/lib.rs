/*
 *  Copyright (c) 2021 Works Applications Co., Ltd.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

//! Clone of [Sudachi](https://github.com/WorksApplications/Sudachi),
//! a Japanese morphological analyzer
//!
//! There is no public API for the initial release.
//! Issue: https://github.com/WorksApplications/sudachi.rs/issues/28
//!
//! Also, there are to mostly
//! [SudachiPy-compatible Python bindings](https://worksapplications.github.io/sudachi.rs/python/).

pub mod analysis;
pub mod config;
pub mod dic;
pub mod error;
pub mod input_text;
pub mod plugin;
pub mod sentence_detector;
pub mod sentence_splitter;

mod hash;
#[cfg(test)]
pub mod test;

pub mod prelude {
    pub use crate::{
        analysis::morpheme::MorphemeList,
        analysis::{Mode, Tokenize},
        error::SudachiError,
        error::SudachiResult,
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
