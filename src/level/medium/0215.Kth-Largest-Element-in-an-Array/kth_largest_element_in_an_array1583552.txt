// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/1583552/rust-minheap-maxheap-0ms-2mb/
use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut k = k as usize;
        let mut minheap = BinaryHeap::with_capacity(k);
        
        nums.into_iter().for_each(|i| minheap.push(Reverse(i)));
        while minheap.len() > k as usize { 
            minheap.pop();  
        }
        minheap.peek().unwrap().0
    }
}