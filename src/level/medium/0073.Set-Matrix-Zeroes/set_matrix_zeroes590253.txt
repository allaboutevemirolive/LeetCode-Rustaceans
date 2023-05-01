// https://leetcode.com/problems/set-matrix-zeroes/solutions/590253/rust-solution-4ms-2-3mb-both-100/
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }

        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut fisrt_row, mut first_column) = (false, false);

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    if i == 0 || j == 0 {
                        fisrt_row = i == 0 || fisrt_row;
                        first_column = j == 0 || first_column;
                    }

                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 0..n {
                    matrix[i][j] = 0;
                }
            }
        }

        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 0..m {
                    matrix[i][j] = 0;
                }
            }
        }

        if fisrt_row {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }

        if first_column {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}