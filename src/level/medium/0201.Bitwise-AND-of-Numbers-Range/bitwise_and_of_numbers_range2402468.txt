// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/2402468/rust-o-1-one-liner-with-explanation/
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        left & right & !((1 << (32 - (right - left).leading_zeros())) - 1)
    }
}