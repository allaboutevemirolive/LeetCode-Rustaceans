// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3442383/bottom-up-dp-rust/
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let chars = s.as_bytes();
        let mut n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..(n-1) {
            dp[i][i+1] = if chars[i] == chars[i+1] {0} else {1};
        }
        for j in 2..n {
            for i in 0..n-j {
                dp[i][i+j] = if (chars[i] == chars[i+j]) {dp[i+1][i+j-1]} else {1+dp[i+1][i+j].min(dp[i][i+j-1])}
            }
        }
        return dp[0][n-1];
    }
}

