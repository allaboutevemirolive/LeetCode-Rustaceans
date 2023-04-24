// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/1162960/rust-o-1-space/
impl Solution {
    pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let (m, n, mut res) = (matrix.len(), matrix[0].len(), 0);
        for y in 0..m {
            let mut row_sum = 0;
            for x in 0..n {
                matrix[y][x] += row_sum;
                row_sum = matrix[y][x];
                matrix[y][x] += if y > 0 { matrix[y - 1][x] } else { 0 };
            }
        }

        for y in 0..m {
            for x in 0..n {
                for y2 in 0..=y {
                    res += (0..=x).filter(|&x2| {
                        let col_sum = if x2 != x { matrix[y][x2] } else { 0 };
                        let row_sum = if y2 != y { matrix[y2][x] } else { 0 };
                        let overlap_sum = if x2 != x && y2 != y { matrix[y2][x2] } else { 0 };
                        matrix[y][x] - col_sum - row_sum + overlap_sum == target
                    }).count();
                }
            }
        }

        res as _
    }
}