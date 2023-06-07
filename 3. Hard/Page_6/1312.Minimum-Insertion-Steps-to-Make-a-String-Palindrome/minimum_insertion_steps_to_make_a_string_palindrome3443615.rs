// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3443615/rust-solution/
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let a = s.into_bytes();
        let n = a.len();
        let mut dp = [vec![0; n], vec![0; n]];
        for len in 1..n {
            dp[0].rotate_left(1);
            dp[0].pop();
            for (i, (aj, ai)) in a[len..].iter().zip(&a).enumerate() {
                if ai != aj {
                    dp[0][i] = dp[1][i].min(dp[1][i + 1]) + 1;
                }
            }
            dp.rotate_left(1);
        }
        dp[1][0]
    }
}
