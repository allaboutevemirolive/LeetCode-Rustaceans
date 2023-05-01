// https://leetcode.com/problems/unique-paths-ii/solutions/1126744/rust-dp-solution/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let row_len = obstacle_grid.len();
        let col_len = obstacle_grid[0].len();
        let mut dp = vec![vec![0; col_len]; row_len];
        for i in 0..row_len {
            for j in 0..col_len {
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                    continue;
                }
                if i == 0 && j == 0 {
                    dp[i][j] = 1;
                } else if i == 0 {
                    dp[i][j] = dp[i][j - 1];
                } else if j == 0 {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }
        dp[row_len - 1][col_len - 1]
    }
}