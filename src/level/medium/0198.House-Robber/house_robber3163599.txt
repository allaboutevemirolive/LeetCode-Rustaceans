// https://leetcode.com/problems/house-robber/solutions/3163599/rust-dp-solution/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut f = vec![0; n + 1];

        f[1] = nums[0];

        for i in 2..n + 1 {
            f[i] = (f[i - 2] + nums[i - 1]).max(f[i - 1]);
        }

        f[n]
    }
}