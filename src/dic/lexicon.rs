pub mod trie;
pub mod word_id_table;
pub mod word_infos;
pub mod word_params;

use nom::le_u32;

use self::trie::Trie;
use self::word_id_table::WordIdTable;
use self::word_infos::{WordInfo, WordInfos};
use self::word_params::WordParams;
use crate::prelude::*;

pub struct Lexicon<'a> {
    trie: Trie,
    word_id_table: WordIdTable<'a>,
    word_params: WordParams<'a>,
    word_infos: WordInfos<'a>,
}

impl<'a> Lexicon<'a> {
    pub fn new(buf: &[u8], original_offset: usize) -> SudachiResult<Lexicon> {
        let mut offset = original_offset;

        let (_rest, trie_size) = parse_size(buf, offset)?;
        offset += 4;
        let (_rest, trie_array) = parse_trie_array(buf, offset, trie_size)?;
        let trie = Trie::new(trie_array, trie_size);
        offset += trie.total_size();

        let (_rest, word_id_table_size) = parse_size(buf, offset)?;
        let word_id_table = WordIdTable::new(buf, word_id_table_size, offset + 4);
        offset += word_id_table.storage_size();

        let (_rest, word_params_size) = parse_size(buf, offset)?;
        let word_params = WordParams::new(buf, word_params_size, offset + 4);
        offset += word_params.storage_size();

        let word_infos = WordInfos::new(buf, offset, word_params.size());

        Ok(Lexicon {
            trie,
            word_id_table,
            word_params,
            word_infos,
        })
    }

    pub fn lookup(&self, input: &[u8], offset: usize) -> SudachiResult<Vec<(u32, usize)>> {
        let result = self.trie.common_prefix_search(input, offset)?;

        let mut l: Vec<(u32, usize)> = Vec::new(); // (word_id, length)
        for item in result {
            let length = item.1;
            for word_id in self.word_id_table.get(item.0)? {
                l.push((word_id, length));
            }
        }

        Ok(l)
    }

    pub fn get_word_info(&self, word_id: usize) -> SudachiResult<WordInfo> {
        self.word_infos.get_word_info(word_id)
    }

    pub fn get_word_param(&self, word_id: usize) -> SudachiResult<(i16, i16, i16)> {
        let left_id = self.word_params.get_left_id(word_id)?;
        let right_id = self.word_params.get_right_id(word_id)?;
        let cost = self.word_params.get_cost(word_id)?;

        Ok((left_id, right_id, cost))
    }
}

named_args!(
    parse_size(offset: usize)<&[u8], u32>,
    do_parse!(
        _seek: take!(offset) >>
        size: le_u32 >>

        (size)
    )
);

named_args!(
    parse_trie_array(offset: usize, trie_size: u32)<&[u8], Vec<u32>>,
    do_parse!(
        _seek: take!(offset) >>
        trie_array: count!(le_u32, trie_size as usize) >>

        (trie_array)
        // TODO: copied? &[u32] from bytes without copy? Java: `bytes.asIntBuffer();`
    )
);
