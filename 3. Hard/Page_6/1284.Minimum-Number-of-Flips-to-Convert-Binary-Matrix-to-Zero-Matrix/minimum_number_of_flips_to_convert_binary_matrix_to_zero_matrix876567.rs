// https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/solutions/876567/rust-translated-0ms-100/
impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashSet, VecDeque};

        const NEIGHBORS: [i32; 6] = [0, 0, 1, 0, -1, 0];
        let mut start = 0;

        let m = mat.len();
        let n = mat[0].len();
        for i in 0..m {
            for j in 0..n {
                start |= mat[i][j] << (i * n + j);
            }
        }
        let mut q = VecDeque::<i32>::new();
        let mut visited = HashSet::<i32>::new();
        q.push_back(start);
        visited.insert(start);
        let mut step = 0;
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let cur = q.pop_front().unwrap();
                if cur == 0 {
                    return step;
                }
                for i in 0..m {
                    for j in 0..n {
                        let mut next = cur;
                        for k in 0..5 {
                            let r = i as i32 + NEIGHBORS[k];
                            let c = j as i32 + NEIGHBORS[k + 1];
                            if r >= 0 && r < m as i32 && c >= 0 && c < n as i32 {
                                next ^= 1 << (r * n as i32 + c);
                            }
                        }
                        if visited.insert(next) {
                            q.push_back(next);
                        }
                    }
                }
            }
            step += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_flips() {
        assert_eq!(Solution::min_flips(vec![vec![0, 0], vec![0, 1]]), 3);
    }
    #[test]
    fn test_min_flips_02() {
        assert_eq!(Solution::min_flips(vec![vec![0]]), 0);
    }

    #[test]
    fn test_min_flips_03() {
        assert_eq!(
            Solution::min_flips(vec![vec![1, 1, 1], vec![1, 0, 1], vec![0, 0, 0]]),
            6
        );
    }

    #[test]
    fn test_min_flips_04() {
        assert_eq!(Solution::min_flips(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
    }
}