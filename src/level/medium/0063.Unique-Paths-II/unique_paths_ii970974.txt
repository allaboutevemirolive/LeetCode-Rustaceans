// https://leetcode.com/problems/unique-paths-ii/solutions/970974/rust-1-dimensional-dp/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        if obstacle_grid[m - 1][n - 1] == 1 { return 0; }
        
        let mut dp = vec![0; n];
        dp[n - 1] = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if obstacle_grid[i][j] == 1 {
                    dp[j] = 0;
                } else {
                    if j + 1 < n {
                        dp[j] += dp[j + 1];
                    }
                }
            }
        }
        dp[0]
    }
}