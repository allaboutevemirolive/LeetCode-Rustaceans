// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/2600003/rust-slide-window/
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        let mut subL = 0;
        let mut sum = 0;
        let mut i = 0;
        for j in 0..nums.len() {  // j is the end of the slide window
            sum += nums[j];
            while sum >= target {
                subL = j - i + 1;
                sum -= nums[i];
                res = i32::min(res, subL as i32);
                i += 1;
            }
        }
        if res == i32::MAX { return 0 }
        res
    }
}