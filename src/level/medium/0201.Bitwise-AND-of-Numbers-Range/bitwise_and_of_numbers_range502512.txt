// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/502512/rust-mask-shift-solution/
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut answer = 0;
        let mut mask = 0x4000_0000;
        for _ in 0..32 {
            if (m & mask) != (n & mask) {
                break;
            }
            answer += m & mask;
            mask >>= 1;
        }
        answer
    }
}