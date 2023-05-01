// https://leetcode.com/problems/find-peak-element/solutions/2170025/binary-search-rust-o-logn-solution/
impl Solution {
 pub fn find_peak_element(nums: Vec<i32>) -> i32 {
  let (mut low, mut high) = (0, nums.len() - 1);
  while low < high {
   let mid = (low + high) / 2;
   // Check if mid is lower than peak
   if nums[mid] < nums[mid + 1] {
    // If so, up level the mid
    low = mid + 1
   } else {
    // Else, down level the mid because mid is higher than peak
    high = mid
   }
  }
  low as i32
 }
}
