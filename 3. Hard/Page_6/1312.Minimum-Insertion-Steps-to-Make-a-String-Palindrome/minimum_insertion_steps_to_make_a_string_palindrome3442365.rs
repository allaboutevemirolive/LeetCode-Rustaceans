// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3442365/rust-dp/
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![vec![0; s.len()]; s.len()];

        for i in 0..s.len() {
            dp[i][i] = 1;
        }

        for len in 2..=s.len() {
            for i in 0..=s.len() - len {
                let j = i + len - 1;
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i + 1][j]);
                }
            }
        }
        s.len() as i32 - dp[0][s.len() - 1]
    }
}