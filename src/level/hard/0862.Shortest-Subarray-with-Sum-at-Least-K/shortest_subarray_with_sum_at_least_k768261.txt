// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/solutions/768261/rust-translated-16ms/
impl Solution {
    pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;

        let n = a.len();
        let mut ans = n as i32 + 1;
        let mut b = vec![0; n + 1];
        for i in 0..n {
            b[i + 1] = b[i] + a[i]
        }
        let mut q = VecDeque::<usize>::new();
        for i in 0..n + 1 {
            while !q.is_empty() && b[i] - b[*q.front().unwrap()] >= k {
                ans = std::cmp::min(ans, (i - q.pop_front().unwrap()) as i32)
            }
            while !q.is_empty() && b[i] <= b[*q.back().unwrap()] {
                q.pop_back();
            }
            q.push_back(i);
        }
        if ans <= n as i32 {
            ans
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_subarray() {
        assert_eq!(Solution::shortest_subarray(vec![1, 2], 4), -1)
    }

    #[test]
    fn test_shortest_subarray_02() {
        assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 3), 3)
    }
}