// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3442323/rust-2d-dp/
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut dp = vec![vec![0; s.len()]; s.len()];
        let s_ar = s.chars().collect::<Vec<_>>();

        for len in 0..s.len() {
            for i in 0..s.len() {
                let j = i+len;

                if j >= s.len() { continue; }
                if i == j { dp[i][j] = 1; }
                else if j-1 >= 0 && i+1 < s.len() {
                    if s_ar[i] == s_ar[j] {
                        dp[i][j] = 2 + dp[i+1][j-1];
                    } else {
                        dp[i][j] = dp[i+1][j].max(dp[i][j-1]);
                    }
                }
            }
        }

        (s.len() - dp[0][s.len()-1]) as i32
    }
}