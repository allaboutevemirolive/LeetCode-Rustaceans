// https://leetcode.com/problems/3sum/solutions/385447/rust-o-n-2/
use std::collections::HashSet;
use std::cmp::{min, max};
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            vec![]
        } else {
            let mut result = HashSet::new();
            (2..nums.len()).for_each(|i| {
                let k = -nums[i];
                Solution::two_sum(&nums[..i], k).into_iter().for_each(|mut v| {
                    v.push(nums[i].clone());
                    v.sort();
                    result.insert(v);
                });
            });
            result.into_iter().collect()
        }
    }

    fn two_sum(nums: &[i32], k: i32) -> Vec<Vec<i32>> {
        let mut seen = HashSet::new();
        let mut result: HashSet<Vec<i32>> = HashSet::new();
        nums.iter().for_each(|c| {
            if seen.contains(&(k - c)) {
                result.insert(vec![min(*c, k-c), max(*c, k-c)]);
            }
            seen.insert(c);
        });
        result.into_iter().collect()
    }
}