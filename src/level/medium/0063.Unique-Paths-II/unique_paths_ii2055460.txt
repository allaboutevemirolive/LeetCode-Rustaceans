// https://leetcode.com/problems/unique-paths-ii/solutions/2055460/rust-short-answer/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; obstacle_grid[0].len() + 1];
        dp[1] = 1;
        for row in obstacle_grid.iter() {
            for i in 1..dp.len() {
                dp[i] = (1 - row[i - 1]) * (dp[i] + dp[i - 1]);
            }
        }
        dp[dp.len() - 1]
    }
}