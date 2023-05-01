// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3008223/rust-solution/
use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty(){
        return 0;
    }

    let mut map = HashMap::new();
    let mut max = 0;

    for (i,c) in s.chars().enumerate(){
        if let Some(&idx) = map.get(&c){
            max = cmp::max(max, map.len());
            map.retain(|_k,v| *v >= idx);
            map.insert(c, i);
        }
        else{
            map.insert(c, i);
        }
    }

    max = cmp::max(max, map.len());
    max as i32
    }
}