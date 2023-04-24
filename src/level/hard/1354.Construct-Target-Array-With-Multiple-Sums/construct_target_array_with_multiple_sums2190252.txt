// https://leetcode.com/problems/construct-target-array-with-multiple-sums/solutions/2190252/rust-binaryheap-simulate-in-reverse/
use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 { return target[0] == 1; }
        // Because everything is strictly positive and increasing,
        // the same number cannot appear twice in valid `target`s.
        let mut total: i32 = target.iter().sum();
        let mut maxheap: BinaryHeap<i32> = BinaryHeap::from(target);
        loop {
            // Unwrap safety: we never add nor remove heap elements, so heap is never empty.
            let mut big = maxheap.peek_mut().unwrap();
            let me: i32 = *big;
            // `maxheap` will never be in invalid state, so this is "if all are 1".
            if me == 1 { return true; }
            let total_except_me = total - me;
            // Needed as a guard against the next step.
            if total_except_me == 1 { return true; }
            // If `i` is the same for multiple consecutive iterations,
            // this will undo them all at once.
            let my_former_value = me % total_except_me;
            // This is how we prevent invalid state.
            if my_former_value == 0 || my_former_value == me {
                // "if sum of other elements is too low or too high"
                return false;
            }
            // "Commit changes"
            *big = my_former_value;
            total = total - me + my_former_value;
        }
    }
}