// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/851869/rust-translated-76ms-100/
impl Solution {
    pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::HashMap;

        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = 0;
        for i in 0..m {
            for j in 1..n {
                matrix[i][j] += matrix[i][j - 1];
            }
        }
        let mut counter = HashMap::<i32, i32>::new();
        for i in 0..n {
            for j in i..n {
                counter.clear();
                counter.insert(0, 1);
                let mut cur = 0;
                for k in 0..m {
                    cur += matrix[k][j] - if i > 0 { matrix[k][i - 1] } else { 0 };
                    ans += counter.get(&(cur - target)).unwrap_or(&0);
                    *counter.entry(cur).or_default() += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_submatrix_sum_target() {
        assert_eq!(
            Solution::num_submatrix_sum_target(
                vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
                0
            ),
            4
        );
    }

    #[test]
    fn test_num_submatrix_sum_target_02() {
        assert_eq!(
            Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0),
            5
        );
    }

    #[test]
    fn test_num_submatrix_sum_target_03() {
        assert_eq!(Solution::num_submatrix_sum_target(vec![vec![904]], 0), 0);
    }

    #[test]
    fn test_num_submatrix_sum_target_04() {
        assert_eq!(
            Solution::num_submatrix_sum_target(
                vec![
                    vec![0, 1, 1, 1, 0, 1],
                    vec![0, 0, 0, 0, 0, 1],
                    vec![0, 0, 1, 0, 0, 1],
                    vec![1, 1, 0, 1, 1, 0],
                    vec![1, 0, 0, 1, 0, 0]
                ],
                0
            ),
            43
        );
    }
}