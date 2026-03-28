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
pub(crate) mod util;

mod hash;
pub mod pos;
#[cfg(test)]
pub mod test;

pub mod prelude {
    pub use crate::{
        analysis::mlist::MorphemeList, analysis::morpheme::Morpheme, analysis::Mode,
        error::SudachiError, error::SudachiResult,
    };
}

extern crate wasm_bindgen;

use std::rc::Rc;

use crate::analysis::stateless_tokenizer::StatelessTokenizer;
use crate::analysis::{Mode, Tokenize};
use crate::config::Config;
use crate::dic::dictionary::JapaneseDictionary;
use crate::dic::storage::{Storage, SudachiDicData};
use serde::Serialize;
use wasm_bindgen::prelude::*;

// embed dictionary binary file
const BYTES: &[u8] = include_bytes!("../../resources/system.dic");

std::thread_local! {
    static DICTIONARY: Rc<JapaneseDictionary> = {
        let config = Config::new_embedded().expect("failed to load embedded config");
        let storage = SudachiDicData::new(Storage::Borrowed(BYTES));
        Rc::new(
            JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage)
                .expect("failed to build embedded dictionary"),
        )
    };
}

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
    DICTIONARY.with(|dict| {
        let tokenizer = StatelessTokenizer::new(dict.clone());
        let mode = match mode {
            TokenizeMode::A => Mode::A,
            TokenizeMode::B => Mode::B,
            TokenizeMode::C => Mode::C,
        };

        let described_morphemes: Vec<_> = tokenizer
            .tokenize(&input, mode, false)
            .unwrap()
            .iter()
            .map(|morpheme| DescribedMorpheme {
                surface: morpheme.surface().to_string(),
                poses: morpheme.part_of_speech().to_vec(),
                normalized_form: morpheme.normalized_form().to_string(),
                reading_form: morpheme.reading_form().to_string(),
                dictionary_form: morpheme.dictionary_form().to_string(),
            })
            .collect();

        serde_json::to_string(&described_morphemes).unwrap()
    })
}
