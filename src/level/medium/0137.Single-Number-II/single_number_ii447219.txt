// https://leetcode.com/problems/single-number-ii/solutions/447219/3-lines-rust-count-bits-repeating-n-times-finding-k-times/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        Self::_single(nums, 3, 1)
    }

    fn _single(nums: Vec<i32>, k: i32, _l: i32) -> i32 {
        (0..std::mem::size_of::<i32>()*8).map(|i| {
            if nums.iter().filter(|n| **n & (1 << i) != 0).count() % k as usize > 0 {
                1 << i } else { 0 } }).sum()
    }
}