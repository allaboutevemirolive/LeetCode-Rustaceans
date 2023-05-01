// https://leetcode.com/problems/minimum-path-sum/solutions/2584552/rust-in-place-dp/
impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for j in 1..n {
            grid[0][j] += grid[0][j-1];
        }
        for i in 1..m {
            grid[i][0] += grid[i-1][0];
            for j in 1..n {
                grid[i][j] += grid[i][j-1].min(grid[i-1][j]);
            }
        }
        grid[m-1][n-1]
    }
}