// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/3131939/go-rust-fast-solution/
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut length, mut total , mut left) = (i32::MAX,0,0);
        for right in 0..nums.len() {
            total += nums[right];
            loop {
                if total >= target {
                    length = length.min((right - left) as i32 + 1);
                    total -= nums[left];
                    left  += 1;
                } else {
                    break
                }
            }
        }
        match length {
            i32::MAX => return 0,
            l => return l,
        }
    } 
}