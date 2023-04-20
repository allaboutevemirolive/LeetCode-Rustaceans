// https://leetcode.com/problems/set-intersection-size-at-least-two/solutions/883105/rust-translated-binaryheap-sort-4ms-100/
impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::<(i32, i32)>::new();
        // pop with right smallest then left largest first
        for v in &intervals {
            heap.push((-v[1], v[0]));
        }
        let mut ans = 0;
        let mut hi = -1;
        let mut lo = -1;

        while let Some((right, left)) = heap.pop() {
            let right = -right;
            if left <= lo {
                continue;
            };
            if left > hi {
                ans += 2;
                hi = right;
                lo = hi - 1;
            } else {
                ans += 1;
                lo = hi;
                hi = right;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_size_two() {
        assert_eq!(
            Solution::intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]),
            3
        );
    }

    #[test]
    fn test_intersection_size_two_02() {
        assert_eq!(
            Solution::intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]),
            5
        );
    }

    #[test]
    fn test_intersection_size_two_03() {
        assert_eq!(
            Solution::intersection_size_two(vec![
                vec![3, 14],
                vec![4, 14],
                vec![3, 9],
                vec![5, 13],
                vec![10, 17],
                vec![8, 20],
                vec![7, 12],
                vec![15, 19],
                vec![11, 17],
                vec![6, 18],
                vec![16, 20],
                vec![2, 18],
                vec![3, 5],
                vec![15, 18],
                vec![9, 12],
                vec![3, 14],
                vec![10, 15],
                vec![1, 13],
                vec![8, 10],
                vec![0, 20]
            ]),
            7
        );
    }
}