// https://leetcode.com/problems/set-matrix-zeroes/solutions/3346991/rust-2-approaches/
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        fn using_additional_memory(matrix: &mut Vec<Vec<i32>>) {
            let mut rows = vec![false; matrix.len()];
            let mut cols = vec![false; matrix[0].len()];
            for i in 0..matrix.len() {
                for j in 0..matrix[0].len() {
                    if matrix[i][j] == 0 {
                        rows[i] = true;
                        cols[j] = true;
                    }
                }
            }
            for i in 0..matrix.len() {
                for j in 0..matrix[0].len() {
                    if rows[i] || cols[j] {
                        matrix[i][j] = 0;
                    }
                }
            }
        }
        fn using_constant_memory(matrix: &mut Vec<Vec<i32>>) {
            let mut col = false;
            for i in 0..matrix.len() {
                if matrix[i][0] == 0 {
                    col = true
                }
                for j in 1..matrix[0].len() {
                    if matrix[i][j] == 0 {
                        matrix[i][0] = 0;
                        matrix[0][j] = 0;
                    }
                }
            }

            for i in 1..matrix.len() {
                for j in 1..matrix[0].len() {
                    if matrix[i][0] == 0 || matrix[0][j] == 0 {
                        matrix[i][j] = 0;
                    }
                }
            }

            if matrix[0][0] == 0 {
                for i in 0..matrix[0].len() {
                    matrix[0][i] = 0;
                }
            }

            if col {
                for i in 0..matrix.len() {
                    matrix[i][0] = 0;
                }
            }
        }
        using_constant_memory(matrix)       
    }
}