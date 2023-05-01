// https://leetcode.com/problems/maximal-square/solutions/600537/simple-rust-dp/
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut max_side = 0;
        let mut dp = vec![vec![0; cols + 1]; rows + 1];

        for j in (0..cols).rev() {
            for i in (0..rows).rev() {
                if matrix[i][j] == '1' {
                    dp[i][j] = 1 + dp[i][j+1].min(dp[i+1][j]).min(dp[i+1][j+1]);
                    max_side = max(max_side, dp[i][j]);
                }
            }
        }

        max_side as i32 * max_side as i32
    }
}