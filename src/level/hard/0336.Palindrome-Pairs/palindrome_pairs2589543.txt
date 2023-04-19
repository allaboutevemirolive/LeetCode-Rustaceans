// https://leetcode.com/problems/palindrome-pairs/solutions/2589543/rust-hashmap-based-solution-is-accepted-no-tle/
use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(ws: Vec<String>) -> Vec<Vec<i32>> {        
        let words: HashMap<Vec<u8>, i32> = ws.iter().enumerate().map(|(i, word)| {
            (word.bytes().collect(), i as i32)
        }).collect();
        
        let reversed_words: HashMap<Vec<u8>, i32> = ws.iter().enumerate().map(|(i, word)| {
            (word.bytes().rev().collect(), i as i32)
        }).collect();
        
        let mut pairs = vec![];
        
        for (word, &index) in &words {
            for i in 0..=word.len() {
                let prefix = &word[0..i];
                if Self::is_palindrome(prefix) {
                    let suffix = &word[i..];
                    
                    if let Some(&rev_index) = reversed_words.get(suffix) {
                        if rev_index != index {
                            pairs.push(vec![rev_index, index]);
                        }
                    }
                }
            }
        }
        
        for (word, &rev_index) in &reversed_words {
            for i in 1..=word.len() {
                let prefix = &word[0..i];
                if Self::is_palindrome(prefix) {
                    let suffix = &word[i..];
                    
                    if let Some(&index) = words.get(suffix) {
                        if rev_index != index {
                            pairs.push(vec![rev_index, index]);
                        }
                    }
                }
            }
        }
     
        pairs
    }
    
    fn is_palindrome(s: &[u8]) -> bool {
        if s.is_empty() {
            return true;
        }
        
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}