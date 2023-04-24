// https://leetcode.com/problems/construct-target-array-with-multiple-sums/solutions/2191707/weird-rust-solution/
pub fn is_possible(target: Vec<i32>) -> bool {
    use std::collections::BinaryHeap;
    let mut current: i32 = target.iter().sum();
    let target_len = target.len() as i32;
    if target_len == 1 {
        return target[0] == 1;
    }
    let mut heap = BinaryHeap::from(target);
    while current > target_len {
        let max = heap.pop().unwrap();
        if (current - max) == 1 {
            return true;
        }
        let remainder = max % (current - max);
        if remainder == max || remainder == 0 {
            return false;
        }
        heap.push(remainder);
        current = current - max + remainder;
    }
    return current == target_len;
}