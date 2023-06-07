// https://leetcode.com/problems/find-k-th-smallest-pair-distance/solutions/3516823/rust-solution/
impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut lo = 0;
        let mut hi = nums[nums.len() - 1] - nums[0];

        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut count = 0;
            let mut left = 0;

            for right in 0..nums.len() {
                while nums[right] - nums[left] > mid {
                    left += 1;
                }
                count += right - left;
            }

            if count >= k as usize {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo        
    }
}