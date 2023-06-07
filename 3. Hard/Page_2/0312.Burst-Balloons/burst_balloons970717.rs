// https://leetcode.com/problems/burst-balloons/solutions/970717/rust-dp-n-3/
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        nums.insert(0, 1);
        nums.push(1);

        let mut c = vec![vec![0; n + 2]; n + 2];
        for l in 1..=n {
            for i in 1..=(n - l + 1) {
                let j = i + l - 1;
                for k in i..=j {
                    c[i][j] = std::cmp::max(
                        c[i][j],
                        c[i][k - 1] + nums[i - 1] * nums[k] * nums[j + 1] + c[k + 1][j],
                    )
                }
            }
        }
        c[1][n]
    }
}