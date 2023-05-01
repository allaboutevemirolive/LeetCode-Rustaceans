// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/2084803/ruby-one-liner-rust-port/
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut l = left;
        let mut r = right;
        let mut mask = -2;
        while l != r {
            l &= mask;
            r &= mask;
            mask *= 2;
        }
        l
    }
}