// https://leetcode.com/problems/construct-target-array-with-multiple-sums/solutions/2190835/rust-binaryheap-and-overflow-check/
use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }
        let mut heap = BinaryHeap::from(target);
        let mut others = -heap.peek().unwrap();
        for &x in heap.iter() {
            match others.checked_add(x) {
                Some(o) => others = o,
                None => return false,
            };
        }
        while heap.peek() != Some(&1) {
            let max = heap.pop().unwrap();
            if others >= max {
                return false;
            }
            let before = (max - 1) % others + 1;
            heap.push(before);
            others = others + before - heap.peek().unwrap();
        }
        true
    }
}