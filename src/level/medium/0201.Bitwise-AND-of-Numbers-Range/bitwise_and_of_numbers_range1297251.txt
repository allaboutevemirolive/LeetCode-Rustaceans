// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/1297251/elegant-using-match-in-rust/
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        match (left, right) {
            (0, 0) => 0,
            (left, right) if left == right => left,
            (left, right) => Self::range_bitwise_and(left >> 1, right >> 1) << 1,
        }
    }
}