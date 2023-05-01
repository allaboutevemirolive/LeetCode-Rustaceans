// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/2183508/rust-solution-for-the-interview-explained/
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let capacity = k as usize;
		// preallocate memory for the capacity k+1, so it doesn't need to be reallocated while function is called
        let mut maxHeap = BinaryHeap::with_capacity(capacity + 1);
        
        for num in nums {
			// Reverse is used because we need a MinHeap, instead of a default behavior
            maxHeap.push(Reverse(num));
            
            if maxHeap.len() > capacity {
                maxHeap.pop();
            }
        }
        
		// peek the top of the heap, unwrap Reversed value from Option,
		// access the actual value from Reverse via .0
        maxHeap.peek().unwrap().0
    }
}