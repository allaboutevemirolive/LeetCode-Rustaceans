// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/solutions/578533/rust-solution-0ms/
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let (mut r, mut s) = (6_i64, 6_i64) ; 
        let x = 10_i64.pow(9) + 7; 
        for i in 1..n {
            let _r = r * 3 + s * 2; 
            let _s = r * 2 + s * 2; 
            r = _r % x; 
            s = _s % x; 
        }
        ((r + s) % x) as i32 
    }
}