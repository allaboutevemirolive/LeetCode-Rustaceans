// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/1763864/rust-solution-faster-than-100/
use std::cmp::min;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut start: usize = 0;
        let mut res: i32 = i32::MAX;
        let mut sum: i32 = 0;
        for end in 0..len {
            sum += nums[end];
            while sum >= target {
                res = min(res,(end-start+1) as i32);
                sum -= nums[start];
                start += 1;
            }
        }
        return if res == i32::MAX {0} else {res};
    }
}