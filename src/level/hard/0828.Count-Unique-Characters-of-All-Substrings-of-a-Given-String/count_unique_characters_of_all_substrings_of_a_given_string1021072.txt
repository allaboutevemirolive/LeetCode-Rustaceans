// https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/solutions/1021072/rust-solution/
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut locs: HashMap<char, VecDeque<usize>> = HashMap::new();
        
        for (i, c) in s.chars().enumerate() {
            if let Some(deq) = locs.get_mut(&c) {
                deq.push_back(i);
            } else {
                let mut deq = VecDeque::new();
                deq.push_back(i);
                locs.insert(c, deq);
            }
        }
        
        for deq in locs.values_mut() {
            deq.push_back(s.len());
        }
        
        let mut solution = 0;
        
        for i in 0..s.len() {
            for deq in locs.values_mut() {
                if *deq.front().unwrap() < i {
                    deq.pop_front();
                }
            }
            
            locs.retain(|_k, v| v.len() > 1);
            
            for deq in locs.values() {
                solution += deq[1] - deq[0];
                solution %= 10e9 as usize + 7;
            }
        }
        
        solution as i32
    }
}