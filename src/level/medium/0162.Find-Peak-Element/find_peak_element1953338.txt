// https://leetcode.com/problems/find-peak-element/solutions/1953338/rust-binary-search/
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {        
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
			// due to integer division, mid+1 must be a valid index
            if nums[mid] < nums[mid+1] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        
        lo as i32
    }
}