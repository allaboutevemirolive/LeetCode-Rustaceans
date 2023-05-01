// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/2076976/rust-binary-search/
impl Solution {
  pub fn find_min(nums: Vec<i32>) -> i32 {
      let n = nums.len();

      let mut left = 0;
      let mut right = n - 1;

      if nums[left] < nums[right] {
        return nums[left];
      }

      while right - left > 1 {
        let mid = left + (right - left)/2;
        if nums[mid] < nums[right] {
          right = mid;
        } else {
          left = mid;
        }
      }
      nums[right]
  }
}