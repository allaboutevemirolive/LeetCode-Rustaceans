// https://leetcode.com/problems/strange-printer/solutions/2003474/rust-dp-solution/
impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        match n {
            0 => 0,
            n @ _ => {
                let mut dp = vec![vec![0; n + 1]; n + 1];
                for len in 1..=n {
                    for i in 0..(n - len + 1) {
                        let j = i + len - 1;
                        dp[i][j] = 1 + dp[i + 1][j];
                        for k in (i + 1)..=j {
                            if s[k] == s[i] {
                                dp[i][j] = std::cmp::min(dp[i][j], dp[i][k - 1] + dp[k + 1][j]);
                            }
                        }
                    }
                }
                dp[0][n - 1]
            }
        } 
    }
}