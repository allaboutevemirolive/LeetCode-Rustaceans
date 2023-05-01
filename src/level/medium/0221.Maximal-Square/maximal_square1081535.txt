// https://leetcode.com/problems/maximal-square/solutions/1081535/rust-solution-dp/
use std::cmp;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut ans = 0;
        
        for row in (0..matrix.len()) {
            for col in (0..matrix[row].len()) {
                if matrix[row][col] == '1' {
                    if row == 0 || col == 0 {
                        dp[row][col] = 1;
                    }
                    else{
                        dp[row][col] = cmp::min(dp[row][col - 1], cmp::min(dp[row - 1][col], dp[row - 1][col - 1])) + 1;
                    }
                    ans = cmp::max(dp[row][col], ans);
                }
            }
        }
        
        ans * ans
    }
}