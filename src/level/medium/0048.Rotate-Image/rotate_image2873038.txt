// https://leetcode.com/problems/rotate-image/solutions/2873038/solution-in-rust/
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                let t = matrix[i][j];
                matrix[i][j] = matrix[n-j-1][i];
                matrix[n-j-1][i] = matrix[n-i-1][n-j-1];
                matrix[n-i-1][n-j-1] = matrix[j][n-i-1];
                matrix[j][n-i-1] = t;
            }
            if n % 2 == 1 {
                let t = matrix[i][n/2];
                matrix[i][n/2] = matrix[n/2][i];
                matrix[n/2][i] = matrix[n-i-1][n/2];
                matrix[n-i-1][n/2] = matrix[n/2][n-i-1];
                matrix[n/2][n-i-1] = t;
            }
        }
    
    }
}