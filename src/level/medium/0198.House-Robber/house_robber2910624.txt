// https://leetcode.com/problems/house-robber/solutions/2910624/rust-solution/
use std::cmp::Ordering::*;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max = vec![0; n];
        for i in 0..n {
            match i.cmp(&2) {
                Less => {
                    max[i] = nums[i];
                }
                Equal => {
                    max[2] = nums[2] + nums[0];
                }
                Greater => {
                    max[i] = nums[i] + i32::max(max[i - 2], max[i - 3]);
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            res = i32::max(max[i], res);
        }
        res
    }
}