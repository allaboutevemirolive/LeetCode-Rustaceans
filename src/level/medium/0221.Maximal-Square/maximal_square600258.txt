// https://leetcode.com/problems/maximal-square/solutions/600258/rust-easy-to-understand-dp-solution/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() { return 0 };
        
        let (row, col) = (matrix.len() + 1, matrix[0].len() + 1);
        let mut dp: Vec<Vec<i32>> = vec![vec![0; col]; row];
        let mut N = 0;
        
        for i in 1..row {
            for j in 1..col {
                if matrix[i-1][j-1] == '1' {
                    dp[i][j] = dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]) + 1;
                    N = N.max(dp[i][j])
                }
            }
        }
        
        return N * N;  
    }
}