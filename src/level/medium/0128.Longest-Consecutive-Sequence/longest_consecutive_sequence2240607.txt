// https://leetcode.com/problems/longest-consecutive-sequence/solutions/2240607/rust-solution/
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let h: HashSet<_> = nums.iter().copied().collect();
        nums.into_iter()
            .filter(|&x| !h.contains(&(x - 1)))
            .map(|x| (x..).take_while(|x| h.contains(x)).count())
            .max()
            .map_or(0, |x| x as i32)
    }
}