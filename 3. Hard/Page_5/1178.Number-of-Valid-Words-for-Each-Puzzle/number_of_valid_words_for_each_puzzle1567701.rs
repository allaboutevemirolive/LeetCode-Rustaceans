// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/solutions/1567701/rust-translated-28ms/
use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut hm = HashMap::new();
        for w in &words {
            *hm.entry(w.bytes().fold(0_u32, |acc, u| acc | 1 << (u - b'a')))
                .or_insert(0) += 1;
        }
        puzzles
            .iter()
            .map(|p| {
                let first = 1 << (p.as_bytes()[0] - b'a');
                let mask = p.bytes().skip(1).fold(0, |acc, u| acc | 1 << (u - b'a'));
                let mut count = *hm.get(&first).unwrap_or(&0);
                let mut submask = mask;
                while submask > 0 {
                    count += hm.get(&(submask | first)).unwrap_or(&0);
                    submask = (submask - 1) & mask;
                }
                count
            })
            .collect()
    }
}