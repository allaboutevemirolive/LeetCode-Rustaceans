// https://leetcode.com/problems/generate-parentheses/solutions/2043166/rust/
use std::collections::HashSet;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut cache = HashSet::new();

        if n == 0 {
            cache.insert("".to_string());
        } else {
            for i in 0..n{
                for open in Solution::generate_parenthesis(i) {
                    for close in Solution::generate_parenthesis(n - i - 1) {
                        cache.insert(format!("({}){}", open, close).to_string());
                    }
                }
            }
        }
        
         cache.into_iter().collect()
    }
}