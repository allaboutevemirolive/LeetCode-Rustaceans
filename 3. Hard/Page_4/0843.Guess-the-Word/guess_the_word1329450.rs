// https://leetcode.com/problems/guess-the-word/solutions/1329450/rust-straightforward-solution/
/**
 * // This is the Master's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct Master;
 * impl Master {
 *     fn guess(word:String)->int;
 * };
 */
use std::collections::HashMap;

impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        let mut remaining = words.iter().collect::<HashSet<_>>();

        for _ in 0..10 {
            let mut frequencies: Vec<HashMap<char, usize>> = (0..6).map(|_| HashMap::default()).collect::<Vec<_>>();
            remaining.iter().copied().for_each(|word| {
                word.chars().enumerate().for_each(|(i, c)| {
                    *frequencies[i].entry(c).or_default() += 1;
                });
            });
            let next_best_guess = remaining.iter().copied().max_by_key(|word| word.chars().enumerate().map(|(i, c)| frequencies[i].get(&c).unwrap()).sum::<usize>()).unwrap();

            let answer = master.guess(next_best_guess.to_string());
            if answer < 0 {
                unreachable!()
            } else if answer == 6 {
                break
            } else {
                remaining = remaining.iter().copied().filter(|word| {
                    next_best_guess.chars().zip(word.chars()).filter(|(a, b)| a == b).count() == answer as usize
                }).collect();
            }
        }
        
    }
}