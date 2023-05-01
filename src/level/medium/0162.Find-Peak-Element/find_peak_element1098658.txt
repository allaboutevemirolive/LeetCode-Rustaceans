// https://leetcode.com/problems/find-peak-element/solutions/1098658/rust-binary-search-explanation/
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mut mid = left + (right - left) / 2;
            if nums[mid + 1] > nums[mid] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}