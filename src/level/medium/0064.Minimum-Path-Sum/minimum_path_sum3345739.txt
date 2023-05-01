// https://leetcode.com/problems/minimum-path-sum/solutions/3345739/rust-dp/
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp = vec![0; n];
        dp[0] = grid[0][0];
        for i in 1..n {
            dp[i] = dp[i - 1] + grid[0][i];
        }        

        for i in 1..m {
            dp[0] = dp[0] + grid[i][0];
            for j in 1..n {
                dp[j] = dp[j].min(dp[j - 1]) + grid[i][j];
            }
        }
        dp[n - 1]
    }
}