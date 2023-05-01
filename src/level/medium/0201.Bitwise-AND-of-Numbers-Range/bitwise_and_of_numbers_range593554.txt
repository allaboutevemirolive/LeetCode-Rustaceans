// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/593554/rust-one-liner-no-loops-0ms-solution/
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        m&!((1<<32-(m^n).leading_zeros())-1)
    }
}