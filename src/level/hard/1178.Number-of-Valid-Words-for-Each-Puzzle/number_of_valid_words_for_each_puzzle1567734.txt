// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/solutions/1567734/rust-solution/
use std::collections::HashMap;
use std::iter::successors;

fn to_bit(c: u8) -> u32 {
    1 << (c - b'a') as u32
}

fn calc_mask(word: &str) -> u32 {
    word.bytes().map(to_bit).fold(0, |x, y| x | y)
}

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut cnt = HashMap::new();
        for word in words {
            *cnt.entry(calc_mask(&word)).or_default() += 1;
        }

        puzzles
            .into_iter()
            .map(|puzzle| {
                let first = to_bit(puzzle.as_bytes()[0]);
                let m = calc_mask(&puzzle) ^ first;
                successors(Some(m), |&s| if s > 0 { Some((s - 1) & m) } else { None })
                    .filter_map(|m| cnt.get(&(m | first)))
                    .sum()
            })
            .collect()
    }
}