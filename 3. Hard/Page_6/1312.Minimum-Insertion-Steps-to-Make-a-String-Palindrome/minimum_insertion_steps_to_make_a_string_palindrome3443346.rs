// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3443346/rust-dp-short-and-concise-solution/
use std::cmp::min;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![0; chars.len()]; chars.len() + 1];
        for i in 2..chars.len() + 1 {
            for j in 0..chars.len() - i + 1 {
                if chars[j] == chars[j + i - 1] {
                    dp[i][j] = dp[i - 2][j + 1];
                } else {
                    dp[i][j] = min(dp[i - 1][j], dp[i - 1][j + 1]) + 1;
                }
            }
        }

        return dp[chars.len()][0];
    }
}