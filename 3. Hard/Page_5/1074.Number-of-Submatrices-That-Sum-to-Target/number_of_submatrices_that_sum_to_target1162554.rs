// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/1162554/rust-translated/
use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        // NOTE: calc matrix prefix sum
        let mut ps = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                ps[i][j] = ps[i - 1][j] + ps[i][j - 1] - ps[i - 1][j - 1] + matrix[i - 1][j - 1];
            }
        }

        let mut res = 0;
        let mut hashmap = HashMap::new();
        // NOTE: for each row range: Subarray Sum Equals K
        for i1 in 1..=m {
            for i2 in i1..=m {
                hashmap.clear();
                hashmap.insert(0, 1);
                for j in 1..=n {
                    let sum = ps[i2][j] - ps[i1 - 1][j];
                    if let Some(prev_count) = hashmap.get(&(sum - target)) {
                        res += prev_count
                    }
                    *hashmap.entry(sum).or_insert(0) += 1;
                }
            }
        }
        res
    }
}