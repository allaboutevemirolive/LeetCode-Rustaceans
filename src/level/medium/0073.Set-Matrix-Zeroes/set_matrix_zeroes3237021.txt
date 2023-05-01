// https://leetcode.com/problems/set-matrix-zeroes/solutions/3237021/rust-solution-easy/
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut x : bool = false;
        let mut y : bool = false;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    if i == 0 && j == 0 {
                        x = true;
                        y = true;
                    }
                    else if i == 0 {
                        x = true;
                    }
                    else if j == 0 {
                        y = true;
                    }
                    else {
                        matrix[i][0] = 0;
                        matrix[0][j] = 0;
                    }
                }
            }
        }

        let mut ans : Vec<Vec<i32>> = matrix.clone();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i == 0 && x {
                    matrix[i][j] = 0;
                }
                if j == 0 && y {
                    matrix[i][j] = 0;
                }

                if ans[0][j] == 0 || ans[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}