// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/755286/rust/
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let (mut m, mut n) = (m, n);
        let mut i = 0;
        while(n > m)
        {
            n >>= 1;
            m >>= 1;
            i += 1;
        }
        return m << i;
    }
}