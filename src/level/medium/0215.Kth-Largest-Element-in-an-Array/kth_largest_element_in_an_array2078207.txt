// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/2078207/rust-one-liner-0ms-2-1/
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        BinaryHeap::from(nums.to_owned()).into_sorted_vec()[nums.len() - k as usize]
    }
}