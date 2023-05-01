// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/2181843/rust-solution/
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(nums);
        loop {
            if let Some(val) = heap.pop() {
                match k > 1 {
                    true => k -= 1,
                    false => break val,
                }
            }
        }
    }
}