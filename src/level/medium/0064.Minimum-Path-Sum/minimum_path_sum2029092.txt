// https://leetcode.com/problems/minimum-path-sum/solutions/2029092/rust-bottom-up-dp/
use std::cmp::min;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        assert!(grid.len() > 0);

        for i in (0..grid.len()).rev() {
            for j in (0..grid[i].len()).rev() {
                if i == grid.len() - 1 && j == grid[i].len() - 1 {
                    continue;
                }
                if i == grid.len() - 1 {
                    grid[i][j] += grid[i][j + 1];
                    continue;
                }
                if j == grid[i].len() - 1 {
                    grid[i][j] += grid[i + 1][j];
                    continue;
                }
                grid[i][j] += min(grid[i + 1][j], grid[i][j + 1]);
            }
        }
        grid[0][0]
    }
}