// https://leetcode.com/problems/find-peak-element/solutions/1660568/rust-heap-iterative-binary-search-solutions/
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut num = nums.clone();
        let mut max = BinaryHeap::from(nums);
        let max_num = max.peek().unwrap();
        num.iter().position(|x| x==max_num).unwrap() as i32
    }
}