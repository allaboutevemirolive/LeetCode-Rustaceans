// https://leetcode.com/problems/rotate-image/solutions/3235638/rust-transponse-reverse/
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        (0..matrix.len()).for_each(|i| {
            (i..matrix[0].len()).for_each(|j| {
                let first = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = first;
            })
        });
        matrix.iter_mut().for_each(|row| row.reverse());
    }
}