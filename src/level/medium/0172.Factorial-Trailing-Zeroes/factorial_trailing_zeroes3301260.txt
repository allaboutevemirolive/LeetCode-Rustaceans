// https://leetcode.com/problems/factorial-trailing-zeroes/solutions/3301260/rust-math/
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let (mut start, mut res) = (5, 0);
        while start <= n {
            res += n/start;
            start*=5;
        }
        res
    }
}