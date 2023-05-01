// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/241109/my-20ms-rust-solution/
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut offset = 0;
        let mut start = m;
        let mut end = n;
        while start != end {
            start >>= 1;
            end >>= 1;
            offset += 1;
        }
        let mut ret = m;
        ret >>= offset;
        ret <<= offset;
        ret
    }
}