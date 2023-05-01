// https://leetcode.com/problems/unique-paths-ii/solutions/3202406/rust-bottom-up-dp/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid.len();
        let m = obstacle_grid[0].len();
        let mut dp = vec![vec![0; m]; n];
        if obstacle_grid[0][0] == 0 { dp[0][0] = 1; }
        for i in 1..m {
            if obstacle_grid[0][i] == 0 {
                dp[0][i] = dp[0][i - 1]
            }
        }
        for i in 1..n {
            if obstacle_grid[i][0] == 0 {
                dp[i][0] = dp[i - 1][0]
            }
        }
        for i in 1..n {
            for j in 1..m {
                if obstacle_grid[i][j] == 0 {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }
        dp[n - 1][m - 1]
    }
}