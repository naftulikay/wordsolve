#[cfg(test)]
mod tests;

use flate2::read::GzDecoder;

use std::collections::BTreeMap;
use std::collections::HashSet;

use std::io::prelude::*;
use std::io::BufReader;

static COMPRESSED_DICT_BYTES: &'static [u8] = include_bytes!("../lib/dictionary.txt.gz");

type CharCount = BTreeMap<char, usize>;

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

pub fn get_word_list(min_size: u8, max_size: u8, contains_chars: String) -> HashSet<String> {
    let reader = BufReader::new(GzDecoder::new(&COMPRESSED_DICT_BYTES[..]));
    let mut result = HashSet::new();

    // transform our char list into a character count map
    let budget = contains_chars.char_count();

    for word in reader
        .lines()
        .into_iter()
        .filter_map(|r| r.ok())
        .map(|s| s.trim().to_string())
        .filter(|word| is_valid_word(min_size, max_size, &budget, word.as_str()))
    {
        result.insert(word);
    }

    result
}

fn is_valid_word(min_size: u8, max_size: u8, budget: &CharCount, word: &str) -> bool {
    // the word length must be g/e min_size and l/e max_size
    if word.len() < (min_size as usize) || word.len() > (max_size as usize) {
        return false;
    }

    // the word must be a subset of the "budget," the CharCount of the allowed characters
    for (chr, count) in word.char_count() {
        if &count > budget.get(&chr).unwrap_or(&0) {
            return false;
        }
    }

    true
}
