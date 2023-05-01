// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/1957332/rust-sliding-window/
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut cur_sum = 0;
        let mut i = 0;

        for j in 0..nums.len() {
            cur_sum += nums[j];
            while cur_sum >= target {
                if res == 0 {
                    res = j - i + 1;
                } else {
                    res = std::cmp::min(res, j - i + 1);
                }

                cur_sum -= nums[i];
                i += 1;
            }
        }

        res as i32
    }
}
