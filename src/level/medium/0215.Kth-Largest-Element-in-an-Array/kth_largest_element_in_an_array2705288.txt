// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/2705288/rust-sort-by/
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));
        nums[k as usize - 1]
    }
}