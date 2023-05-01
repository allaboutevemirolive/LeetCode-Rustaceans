// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/2180542/rust-using-built-in-quickselect/
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        *nums.select_nth_unstable(n - k as usize).1
    }
}