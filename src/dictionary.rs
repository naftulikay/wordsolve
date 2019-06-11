#[cfg(test)]
mod tests;

use flate2::read::GzDecoder;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use std::io::prelude::*;
use std::io::BufReader;

static COMPRESSED_DICT_BYTES: &'static [u8] = include_bytes!("../lib/dictionary.txt.gz");

/// A count of characters for a given string.
type CharCount = BTreeMap<char, usize>;

/// A dictionary of words of a given size.
pub struct Dictionary(WordSet);

/// A set of unique dictionary words.
pub type WordSet = BTreeSet<String>;

impl Dictionary {
    /// Construct a new dictionary containing words whose length falls in range.
    ///
    /// This function uses a compiled-in byte array of gzipped, line-delimited dictionary words. It
    /// loops over all of them, selecting ones whose length falls in the specified range. This
    /// operation is somewhat CPU intensive, so cache a `Dictionary` value as long as possible to
    /// eliminate duplicative CPU work.
    pub fn new(min_size: u8, max_size: u8) -> Self {
        Dictionary(
            BufReader::new(GzDecoder::new(&COMPRESSED_DICT_BYTES[..]))
                .lines()
                .into_iter()
                .filter_map(|r| r.ok())
                .map(|s| s.trim().to_string())
                .filter(|word| is_valid_word_size(min_size, max_size, word.as_str()))
                .collect(),
        )
    }

    /// Filter this dictionary, returning only words which satisfy the allowed characters.
    ///
    /// Note that allowed characters can include more than one occurrence of a character.
    pub fn solutions(&self, allowed_chars: &str) -> WordSet {
        let budget = allowed_chars.char_count();

        self.0
            .iter()
            .filter(|word| is_valid_word_chars(&budget, word.as_str()))
            .map(|word| word.clone())
            .collect()
    }
}

trait CharCounter {
    fn char_count(&self) -> CharCount;
}

impl CharCounter for str {
    fn char_count(&self) -> CharCount {
        let mut count = CharCount::new();

        self.chars().for_each(|c| *count.entry(c).or_insert(0) += 1);

        count
    }
}

impl CharCounter for String {
    fn char_count(&self) -> CharCount {
        let mut count = CharCount::new();

        self.chars().for_each(|c| *count.entry(c).or_insert(0) += 1);

        count
    }
}

/// A filter function for determining whether a given word's length is within the allowed range.
#[inline]
fn is_valid_word_size(min_size: u8, max_size: u8, word: &str) -> bool {
    word.len() >= (min_size as usize) && word.len() <= (max_size as usize)
}

/// A filter function for determining whether a given word matches the character count criteria.
#[inline]
fn is_valid_word_chars(budget: &CharCount, word: &str) -> bool {
    for (chr, count) in word.char_count() {
        if &count > budget.get(&chr).unwrap_or(&0) {
            return false;
        }
    }

    true
}
