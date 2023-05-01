// https://leetcode.com/problems/factorial-trailing-zeroes/solutions/1270810/rust-one-liner/
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        (5..=n)
            .filter(|x| 1_220_703_125 % x == 0)
            .fold(0, |acc, cur| acc + n / cur)
    }
}