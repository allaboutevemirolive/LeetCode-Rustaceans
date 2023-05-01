// https://leetcode.com/problems/house-robber/solutions/3281855/rust-1ms/
use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = max(nums[0], nums[1]);
        for i in 2..nums.len() {
            dp[i] = max(dp[i - 1], dp[i - 2] + nums[i]);
        }
        dp[nums.len() - 1]
    }
}