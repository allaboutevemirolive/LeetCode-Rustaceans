// https://leetcode.com/problems/find-k-th-smallest-pair-distance/solutions/597586/rust-8ms-binary-search-solution/
use std::ops::{Add, Div};

impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        binary_search(0, 1000000, | distance |{
            nums
                .iter()
                .enumerate()
                .map(|(i, x)| nums.upper_bound(&(x + distance)) - i - 1)
                .fold(0, |a, b| a + b)
            >= k as usize
        })
    }
}

pub fn binary_search<T>(left: T, right: T, predicate: impl Fn(T) -> bool) -> T
    where T: Add<Output=T> + Div<Output=T> + Ord + From<u8> + Copy
{
    let mut l = left;
    let mut r = right;
    while l < r {
        let mid = (l + r) / T::from(2);
        if predicate(mid) {
            r = mid;
        } else {
            l = mid + T::from(1);
        }
    }
    l
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T> BinarySearch<T> for [T] where
    T: Add<Output=T> + Div<Output=T> + Ord + From<u8> + Copy {
    fn lower_bound(&self, x: &T) -> usize {
        binary_search(0, self.len(), |i| self[i] >= *x)
    }
    fn upper_bound(&self, x: &T) -> usize {
        binary_search(0, self.len(), |i| self[i] > *x)
    }
}