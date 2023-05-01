// https://leetcode.com/problems/rotate-image/solutions/2677766/rust-solution/
     impl Solution {
            pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
                let n = matrix.len();
                //For any matrix to be rotatable it must be a square matrix
                // i.e. no. of rows == no. of cols == size
                for i in 0..n {
                    for j in 0..n {
                        if (i > j) {
                            let temp = matrix[i][j];
                            matrix[i][j] = matrix[j][i];
                            matrix[j][i] = temp;
                        }
                    }
                }
                //Reversing the Matrix
                for row in 0..n {
                    let mut left = 0;
                    let mut right = n - 1;
                    while (left < right) {
                        let temp = matrix[row][left];
                        matrix[row][left] = matrix[row][right];
                        matrix[row][right] = temp;
                        left += 1;
                        right -= 1;
                    }
                }
            }
        }