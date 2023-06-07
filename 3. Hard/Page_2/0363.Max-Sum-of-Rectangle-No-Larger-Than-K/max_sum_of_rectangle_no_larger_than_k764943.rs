// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/solutions/764943/rust-translated/
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::BTreeSet;

        let mut m = matrix.len();
        if m == 0 {
            return 0;
        };
        let mut n = matrix[0].len();
        let col_bigger = n > m;
        if !col_bigger {
            std::mem::swap(&mut m, &mut n)
        };
        let mut ans = std::i32::MIN;
        for i in 0..m {
            let mut array = vec![0; n];
            for j in (0..=i).rev() {
                let mut val = 0;
                let mut set = BTreeSet::<i32>::new();
                set.insert(0);
                for k in 0..n {
                    array[k] += if col_bigger {
                        matrix[j][k]
                    } else {
                        matrix[k][j]
                    };
                    val += array[k];
                    if let Some(&x) = set.range(val - target..).next() {
                        ans = std::cmp::max(ans, val - x);
                    }
                    set.insert(val);
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
    fn test_max_sum_submatrix() {
        assert_eq!(
            Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2),
            2
        )
    }

    #[test]
    fn test_max_sum_submatrix_02() {
        assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2 - 1]], 2), 2)
    }
}