// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/2875177/rust-249ms-no-extra-memory/
impl Solution {
    pub fn num_submatrix_sum_target(mut a: Vec<Vec<i32>>, target: i32) -> i32 {
        let n = a.len();
        let m = match a.get(0) {
            Some(row) => row.len(),
            None => 0,
        };

        for i in 0..n {
            for j in 0..m {
                if i > 0 && j > 0 { a[i][j] -= a[i - 1][j - 1]; } 
                if i > 0 { a[i][j] += a[i - 1][j]; } 
                if j > 0 { a[i][j] += a[i][j - 1]; }
            }
        }

        let mut res = 0;
        for y1 in 0..m {
            for x1 in 0..n {
                for y2 in 0..y1 + 1 {
                    for x2 in 0..x1 + 1 {
                        let sum = if x2 == 0 && y2 == 0 {
                            a[x1][y1]
                        } else if x2 == 0 {
                            a[x1][y1] - a[x1][y2 - 1]
                        } else if y2 == 0 {
                            a[x1][y1] - a[x2 - 1][y1]
                        } else {
                            a[x1][y1] - a[x1][y2 - 1] - a[x2 - 1][y1] + a[x2 - 1][y2 - 1]
                        };

                        if sum == target {
                            res += 1;
                        }
                    }
                }
            }
        }
        res
    }
}
