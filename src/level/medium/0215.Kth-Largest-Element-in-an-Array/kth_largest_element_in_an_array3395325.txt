// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/3395325/rust-min-heap-solution/
use std::collections::BinaryHeap;
use std::cmp::Reverse;



impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut pq: BinaryHeap<Reverse<&i32>> = BinaryHeap::new();
        for v in nums.iter() {
            pq.push(Reverse(v));
            if pq.len() as i32 > k {
                pq.pop();
            }
        }

        return *pq.pop().unwrap().0;
    }
}
