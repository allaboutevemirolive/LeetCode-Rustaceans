// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/226871/rust-recursive-solution/
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        if m == 0 || n == 0 {
            return 0;
        }
        if n == m {
            return n;
        }
        
        // otherwise, [m,n] must contain an even integer => last digit of result is 0
        return Solution::range_bitwise_and(m/2,n/2) << 1;
    }
}