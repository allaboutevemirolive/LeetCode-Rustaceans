// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/2301070/o-n-4-and-o-n-3-solutions-rust/
pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 { // O(n^4)
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut prefix = vec![vec![0; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            prefix[i + 1][j + 1] = matrix[i][j] + prefix[i + 1][j] + prefix[i][j + 1] - prefix[i][j];
        }
    }
    let mut ans = 0;
    for x1 in 0..m {
      for x2 in x1..m {
        for y1 in 0..n {
          for y2 in y1..n {
            if prefix[x2 + 1][y2 + 1] - prefix[x2 + 1][y1] - prefix[x1][y2 + 1] + prefix[x1][y1] == target {
              ans += 1;
            }
          }
        }
      }
    }
    ans
  }