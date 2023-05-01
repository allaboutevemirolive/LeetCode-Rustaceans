// https://leetcode.com/problems/house-robber/solutions/2951503/rust-1ms-2-1-mb/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let (mut prev, mut last) = (0, 0);
        for curr in nums {
            let temp = max(prev + curr, last);
            prev = last;
            last = temp;
        }
        last
    }
}