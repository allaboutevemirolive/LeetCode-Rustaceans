// https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero/solutions/3162322/rust-optimized-o-n-2/
use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::with_capacity(10000);
        let mut res = 0;
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                let x = nums[i] & nums[j];
                map.entry(x).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        for i in nums {
            for (k ,v) in &map {
                if i & k == 0 {
                    res += v;
                }
            } 
        }
        res
    }
}