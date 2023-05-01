// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/3223934/rust-implementation/
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = (k) as usize;

        let mut heap = nums.into_iter().fold(BinaryHeap::new(), |mut h, n| {
            h.push(n);
            h
        });

        let mut result: Option<i32> = None;
        for _ in 0..k {
            result = heap.pop();
        }

        result.unwrap()
    }
}