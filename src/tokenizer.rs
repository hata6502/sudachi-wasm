use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use crate::dic::category_type::CategoryType;
use crate::dic::grammar::Grammar;
use crate::dic::header::Header;
use crate::dic::lexicon::Lexicon;
use crate::lattice::node::Node;
use crate::lattice::Lattice;
use crate::morpheme::Morpheme;
use crate::plugin::oov::{self, OovProviderPlugin};
use crate::plugin::path_rewrite::{self, PathRewritePlugin};
use crate::prelude::*;
use crate::utf8inputtext::{Utf8InputText, Utf8InputTextBuilder};

/// Able to tokenize Japanese text
pub trait Tokenize {
    /// Break text into `Morpheme`s
    fn tokenize(&self, input: &str, mode: Mode, enable_debug: bool)
        -> SudachiResult<Vec<Morpheme>>;
}

/// Tokenizes Japanese text
pub struct Tokenizer<'a> {
    pub grammar: Grammar<'a>,
    pub lexicon: Lexicon<'a>,
    oov_provider_plugins: Vec<Box<dyn OovProviderPlugin>>,
    path_rewrite_plugins: Vec<Box<dyn PathRewritePlugin>>,
}

/// Unit to split text
///
/// Some examples:
/// ```text
/// A：選挙/管理/委員/会
/// B：選挙/管理/委員会
/// C：選挙管理委員会
///
/// A：客室/乗務/員
/// B：客室/乗務員
/// C：客室乗務員
///
/// A：労働/者/協同/組合
/// B：労働者/協同/組合
/// C：労働者協同組合
///
/// A：機能/性/食品
/// B：機能性/食品
/// C：機能性食品
/// ```
///
/// See [Sudachi documentation](https://github.com/WorksApplications/Sudachi#the-modes-of-splitting)
/// for more details
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Short
    A,

    /// Middle (similar to "word")
    B,

    /// Named Entity
    C,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(Mode::A),
            "B" | "b" => Ok(Mode::B),
            "C" | "c" => Ok(Mode::C),
            _ => Err("Mode must be one of \"A\", \"B\", or \"C\" (in lower or upper case)."),
        }
    }
}

impl<'a> Tokenizer<'a> {
    /// Create `Tokenizer` from the raw bytes of a Sudachi dictionary.
    pub fn from_dictionary_bytes(dictionary_bytes: &'a [u8]) -> SudachiResult<Tokenizer<'a>> {
        let (_rest, _header) = Header::new(&dictionary_bytes[..Header::STORAGE_SIZE])?;
        let mut offset = Header::STORAGE_SIZE;

        let grammar = Grammar::new(dictionary_bytes, offset)?;
        offset += grammar.storage_size;

        let lexicon = Lexicon::new(dictionary_bytes, offset)?;

        // todo: load plugins
        let oov_provider_plugins = oov::get_oov_plugins(&grammar)?;
        if oov_provider_plugins.is_empty() {
            return Err(SudachiError::NoOOVPluginProvided);
        }
        let path_rewrite_plugins = path_rewrite::get_path_rewrite_plugins(&grammar)?;

        Ok(Tokenizer {
            grammar,
            lexicon,
            oov_provider_plugins,
            path_rewrite_plugins,
        })
    }
}

/// Return bytes of a `dictionary_path`
pub fn dictionary_bytes_from_path<P: AsRef<Path>>(dictionary_path: P) -> SudachiResult<Vec<u8>> {
    let dictionary_path = dictionary_path.as_ref();
    let dictionary_stat = fs::metadata(&dictionary_path)?;
    let mut dictionary_file = File::open(dictionary_path)?;
    let mut dictionary_bytes = Vec::with_capacity(dictionary_stat.len() as usize);
    dictionary_file.read_to_end(&mut dictionary_bytes)?;

    Ok(dictionary_bytes)
}

impl<'a> Tokenizer<'a> {
    fn build_lattice(&self, input: &Utf8InputText) -> SudachiResult<Lattice> {
        let input_bytes = input.modified.as_bytes();
        let mut lattice = Lattice::new(&self.grammar, input_bytes.len());
        for (i, _) in input_bytes.iter().enumerate() {
            if !input.can_bow(i) || !lattice.has_previous_node(i) {
                continue;
            }

            let mut has_word = false;
            for (word_id, end) in self.lexicon.lookup(&input_bytes, i)? {
                if (end < input_bytes.len()) && !input.can_bow(end) {
                    continue;
                }
                has_word = true;
                let (left_id, right_id, cost) = self.lexicon.get_word_param(word_id as usize)?;
                let node = Node::new(left_id, right_id, cost, word_id);
                lattice.insert(i, end, node)?;
            }

            // OOV
            if !input
                .get_char_category_types(i)
                .contains(&CategoryType::NOOOVBOW)
            {
                for oov_provider in &self.oov_provider_plugins {
                    for node in oov_provider.get_oov(&input, i, has_word)? {
                        has_word = true;
                        lattice.insert(node.begin, node.end, node)?;
                    }
                }
            }
            if !has_word {
                // use last oov_provider as default
                for node in self
                    .oov_provider_plugins
                    .last()
                    .unwrap()
                    .get_oov(&input, i, has_word)?
                {
                    has_word = true;
                    lattice.insert(node.begin, node.end, node)?;
                }
            }

            if !has_word {
                panic!("no morpheme found at {}", i);
            }
        }
        lattice.connect_eos_node()?;

        Ok(lattice)
    }

    fn split_path(&self, path: Vec<Node>, mode: Mode) -> SudachiResult<Vec<Node>> {
        if mode == Mode::C {
            return Ok(path);
        }

        let mut new_path = Vec::with_capacity(path.len());
        for node in path {
            let word_info = node
                .word_info
                .clone()
                .ok_or(SudachiError::MissingWordInfo)?;
            let word_ids = match mode {
                Mode::A => &word_info.a_unit_split,
                Mode::B => &word_info.b_unit_split,
                _ => unreachable!(),
            };

            if word_ids.len() >= 2 {
                new_path.push(node);
            } else {
                let mut offset = node.begin;
                for wid in word_ids {
                    let mut n = Node::new(0, 0, 0, *wid);
                    n.fill_word_info(&self.lexicon)?;
                    let length = n
                        .word_info
                        .clone()
                        .ok_or(SudachiError::MissingWordInfo)?
                        .head_word_length as usize;
                    n.set_range(offset, offset + length);
                    new_path.push(n);

                    offset += length;
                }
            }
        }

        new_path.shrink_to_fit();
        Ok(new_path)
    }
}

impl<'a> Tokenize for Tokenizer<'a> {
    fn tokenize(
        &self,
        input: &str,
        mode: Mode,
        enable_debug: bool,
    ) -> SudachiResult<Vec<Morpheme>> {
        let builder = Utf8InputTextBuilder::new(input, &self.grammar);
        // todo: plugin: input text
        let input = builder.build();

        let lattice = self.build_lattice(&input)?;

        if enable_debug {
            println!("=== Lattice dump:");
            lattice.dump(&self.grammar)?;
        };

        let mut path = lattice.get_best_path()?;

        // fill word_info to safry unwrap during path_rewrite and split_path
        // todo: remove this process
        for node in &mut path {
            node.fill_word_info(&self.lexicon)?;
        }

        if enable_debug {
            println!("=== Before Rewriting:");
            println!("{:?}", path);
        };

        for plugin in &self.path_rewrite_plugins {
            path = plugin.rewrite(&input, path, &lattice)?;
        }
        let path = self.split_path(path, mode)?;

        if enable_debug {
            println!("=== After Rewriting:");
            println!("{:?}", path);
        };

        path.iter()
            .map(|node| Morpheme::new(&node, &self.grammar, &self.lexicon))
            .collect::<SudachiResult<Vec<_>>>()
    }
}
