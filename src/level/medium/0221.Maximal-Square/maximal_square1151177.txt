// https://leetcode.com/problems/maximal-square/solutions/1151177/rust-bottom-up-dp/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        // dp(i, j) = min { dp(i − 1, j − 1), dp(i − 1, j), dp(i, j − 1) } + 1
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut max = 0;
        for i in 1..=m {
            for j in 1..=n {
                if matrix[i - 1][j - 1] == '1' {
                    let v = [dp[i - 1][j - 1], dp[i - 1][j], dp[i][j - 1]]
                        .iter()
                        .min()
                        .unwrap()
                        + 1;
                    dp[i][j] = v;
                    max = max.max(v);
                }
            }
        }
        max * max
    }
}