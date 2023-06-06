// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/3602455/rust-heapsort-alternative-implementation/
use std::collections::BinaryHeap;
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return true;
        }

        // Try to avoid sorting the entire array
        // Heapify the array
        let mut heap = BinaryHeap::from(arr);

        // Get the difference of the max 2 elements
        let max_element = heap.pop().unwrap();
        let mut prev = heap.pop().unwrap();
        let diff = max_element - prev;

        // Check every element is diff less than the last
        while let Some(element) = heap.pop() {
            if element != prev - diff {
                return false;
            } else {
                prev = element;
            }
        }
        true
    }
}