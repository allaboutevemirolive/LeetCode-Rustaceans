// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3465977/rust-dp/
impl Solution {
    fn min_insertions(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];

        for gap in 1..n {
            for i in 0..(n - gap) {
                let j = i + gap;

                dp[i][j] = if s[i..=i] == s[j..=j] {
                    dp[i + 1][j - 1]
                } else {
                    dp[i + 1][j].min(dp[i][j - 1]) + 1
                };
            }
        }

        dp[0][n - 1]
    }
}