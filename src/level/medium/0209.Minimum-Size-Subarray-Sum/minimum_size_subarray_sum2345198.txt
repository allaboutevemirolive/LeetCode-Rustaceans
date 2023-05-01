// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/2345198/rust-0ms-straightforward/
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min = std::i32::MAX;
        let mut sum = 0;
        let mut left = 0;
        
        for right in 0..nums.len() {
            sum += nums[right];
            while sum >= target {
                min = std::cmp::min(min, (right + 1 - left) as i32);
                sum -= nums[left];
                left += 1;
            }
        }
        if min == std::i32::MAX {0} else {min}
    }
}