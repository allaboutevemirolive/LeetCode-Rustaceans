// https://leetcode.com/problems/longest-consecutive-sequence/solutions/2239265/rust-o-n/
use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut max = 0;
        for num in nums {
            if !map.contains_key(&num) {
                let left = map.get(&(num - 1)).unwrap_or(&0);
                let right = map.get(&(num + 1)).unwrap_or(&0);
                let sum = left + right + 1;
                let left = num - *left;
                let right = num + *right;
                map.insert(num, sum);
                map.insert(left, sum);
                map.insert(right, sum);
                max = std::cmp::max(max, sum);
            }
        }
        max
    }
}