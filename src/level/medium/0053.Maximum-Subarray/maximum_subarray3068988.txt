// https://leetcode.com/problems/maximum-subarray/solutions/3068988/rust-simple/
impl Solution {
    pub fn max_sub_array(mut nums: Vec<i32>) -> i32 {
        let (mut res, mut n) = (nums[0], nums.len());

        for i in 1..n {
            nums[i] = nums[i].max(nums[i] + nums[i - 1]);
            res = nums[i].max(res)
        }

        res
    }
}