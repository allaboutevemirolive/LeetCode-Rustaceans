// https://leetcode.com/problems/factorial-trailing-zeroes/solutions/2258913/rust-log-n-solution/
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        if n == 0 {
            0
        } else {
            n / 5 + Solution::trailing_zeroes(n/5)
        }
    }
}