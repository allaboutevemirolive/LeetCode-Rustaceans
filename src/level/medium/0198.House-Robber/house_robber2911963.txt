// https://leetcode.com/problems/house-robber/solutions/2911963/rust-dp-solution/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // Edge cases.
        if nums.is_empty() {
            return 0;
        }
        else if nums.len() == 1 {
            return nums[0];
        }
        else if nums.len() == 2 {
            return nums[0].max(nums[1]);
        }
        else if nums.len() == 3 {
            return nums[1].max((nums[0] + nums[2]));
        }

        let length = nums.len();
        let mut dp = Vec::new();

        // Optimization.
        dp.push(nums[0]);
        dp.push(nums[1]);
        dp.push(nums[0] + nums[2]);

        for i in 3..length {
            dp.push(nums[i] + dp[i - 2].max(dp[i - 3]));
        }

        return dp[length - 1].max(dp[length - 2]);
    }
}