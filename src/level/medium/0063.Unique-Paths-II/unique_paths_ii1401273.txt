// https://leetcode.com/problems/unique-paths-ii/solutions/1401273/simple-rust-dp-0ms/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (R, C) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; C + 1]; R + 1];

        dp[1][1] = 1;

        for row in 1..=R {
            for col in 1..=C {
                if obstacle_grid[row-1][col-1] == 1 {
                    dp[row][col] = 0;
                } else {
                    dp[row][col] = dp[row][col].max(dp[row-1][col] + dp[row][col-1]);
                }
            }
        }

        dp[R][C]
    }
}