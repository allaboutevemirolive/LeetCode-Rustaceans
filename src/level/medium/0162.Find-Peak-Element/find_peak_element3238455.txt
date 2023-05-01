// https://leetcode.com/problems/find-peak-element/solutions/3238455/python3-golang-rust-binary-search/
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;
        
        while left < right - 1 {
            let mid: i32 = left + (right - left) / 2;
            
            if nums[mid as usize] > nums[mid as usize + 1] && nums[mid as usize] > nums[mid as usize - 1] {
                return mid
            }
            if nums[mid as usize] < nums[mid as usize + 1] {
                left = mid + 1
            } else {
                right = mid - 1
            }
        }
            
        if nums[left as usize] >= nums[right as usize] {
            return left
        } else {
            return right
        }
    }
}