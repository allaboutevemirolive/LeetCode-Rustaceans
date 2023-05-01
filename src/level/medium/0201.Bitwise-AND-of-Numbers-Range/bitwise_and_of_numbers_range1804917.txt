// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/1804917/rust-o-1-bit-manipulation/
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut left = left;
        let mut right = right;
        let mut shift = 0;
        while left != right {
            left = left >> 1;
            right = right >> 1;
            shift += 1;
        }
        return left << shift;
    }
}