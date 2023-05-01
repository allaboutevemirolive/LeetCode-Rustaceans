// https://leetcode.com/problems/rotate-image/solutions/2837152/rust-0ms/
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        for i in 0..matrix.len() {
            for j in i + 1..matrix[0].len() {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }
}