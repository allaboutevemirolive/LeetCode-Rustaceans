// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/3307721/sliding-window-rust/
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut current_sum = nums[0];
        
        let mut answer = usize::MAX;
        loop {
            while current_sum < target {
                if end == n-1 {
                    return if answer == usize::MAX {
                        0
                    } else {
                        answer as i32
                    };
                }
                end += 1;
                current_sum += nums[end];
            }
            while current_sum >= target {
                let length = end - start + 1;
                if answer > length {
                    answer = length;
                }
                current_sum -= nums[start];
                start += 1;
            }
        }
    }
}