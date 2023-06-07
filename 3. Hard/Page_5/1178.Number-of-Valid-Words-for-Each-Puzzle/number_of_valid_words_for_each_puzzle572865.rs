// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/solutions/572865/rust-48ms-bitmask-solution/
use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let map = build_word_map(&words);
        puzzles
            .iter()
            .map(|puzzle| {
                let used_chars = count_used_chars(puzzle);
                sum_bits(&map, used_chars, 0, puzzle.bytes().nth(0).unwrap() - b'a') as i32
            })
            .collect()
    }
}

fn build_word_map(words: &Vec<String>) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for word in words {
        let used_chars = count_used_chars(word);
        *map.entry(used_chars).or_insert(0) += 1;
    }
    map
}

fn count_used_chars(word: &str) -> usize {
    let mut bits = 0;
    for chr in word.bytes() {
        let index = chr - b'a';
        bits |= 1 << index;
    }
    bits
}

fn sum_bits(map: &HashMap<usize, usize>, bits: usize, index: usize, first_char: u8) -> usize {
    if index >= 26 {
        return *map.get(&bits).unwrap_or(&0);
    }

    let bit = 1 << index;
    if (bits & bit) != 0 && (1 << first_char) != bit {
        sum_bits(map, bits ^ bit, index + 1, first_char)
            + sum_bits(map, bits, index + 1, first_char)
    } else {
        sum_bits(map, bits, index + 1, first_char)
    }
}