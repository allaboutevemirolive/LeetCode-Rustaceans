// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/1514988/one-line-rust-solution-using-leading-zeros/
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        right & !((1_i32 << (32-(left ^ right).leading_zeros())).wrapping_sub(1))
    }
}