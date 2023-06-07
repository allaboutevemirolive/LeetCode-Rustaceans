// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3442504/rust-dp-solution/
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let n = s.len();
        let chars_s: Vec<char> = s.chars().collect();
        let chars_r: Vec<char> = s.chars().rev().collect();
        let mut f = vec![vec![0; n + 1]; n + 1];

        for i in 1..n + 1 {
            for j in 1..n + 1 {
                if chars_s[i - 1] == chars_r[j - 1] {
                    f[i][j] = f[i - 1][j - 1] + 1;
                } else {
                    f[i][j] = f[i - 1][j].max(f[i][j - 1]);
                }
            }
        }

        n as i32 - f[n][n]
    }
}