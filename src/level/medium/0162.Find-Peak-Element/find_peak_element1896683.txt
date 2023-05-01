// https://leetcode.com/problems/find-peak-element/solutions/1896683/rust-binary-search-0ms-optimal/
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut right = n - 1;

        while left <= right {
            let mid = left +  (right - left) / 2;
            if (mid == 0 || nums[mid - 1] < nums[mid])
                && (mid == n - 1 || nums[mid + 1] < nums[mid]) {
                return mid as i32;
            }

            if mid > 0 && nums[mid - 1] > nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return -1;
    }
}