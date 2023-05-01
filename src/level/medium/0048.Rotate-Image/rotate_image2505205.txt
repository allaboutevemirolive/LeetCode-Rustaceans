// https://leetcode.com/problems/rotate-image/solutions/2505205/rust-solution/
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        // transpose matrix
        for i in 0..n - 1 {
            for j in i + 1..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        // reverse
        matrix.iter_mut().for_each(|row| row.reverse());
    }
}