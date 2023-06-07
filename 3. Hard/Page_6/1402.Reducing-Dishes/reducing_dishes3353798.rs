// https://leetcode.com/problems/reducing-dishes/solutions/3353798/rust-iterator-0ms/
impl Solution {
    pub fn max_satisfaction(mut s: Vec<i32>) -> i32 {
        let mut sum = 0;
        s.sort_unstable();
        s.into_iter().rev().map(|v| { sum += v; sum }).take_while(|v| *v > 0).sum()
    }
}