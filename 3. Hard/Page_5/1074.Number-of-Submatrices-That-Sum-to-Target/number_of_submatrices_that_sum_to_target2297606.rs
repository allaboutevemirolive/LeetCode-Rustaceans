// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/2297606/rust-prefix-sum-dp-hashmap-for-o-m-n-n/
pub fn num_submatrix_sum_target_old(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
    let mut ans = 0;        
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            dp[i + 1][j + 1] = dp[i + 1][j] + dp[i][j + 1] - dp[i][j] + matrix[i][j];
            for x in 0..i + 1 {
                for y in 0..j + 1 {
                    if dp[i + 1][j + 1] - dp[i + 1][y] - (dp[x][j + 1] - dp[x][y]) == target {
                        ans += 1;
                    }
                }
            }
        }
    }
    ans
}