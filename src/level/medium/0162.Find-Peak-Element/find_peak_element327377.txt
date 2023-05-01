// https://leetcode.com/problems/find-peak-element/solutions/327377/rust-binary-search/
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
		
		// Invariant : peak element is in interval [l, r]
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = l + ((r - l) >> 1);
            if nums[mid] < nums[mid + 1] {
			          // invariant may be broken, the peak ele may be nums[mid+1], it's fine, as from 
					  // now on, r will keep reducing, and then break the while loop
                l = mid + 1; 
            } else {
			    r = mid;    // invariant is kept
            }
        }
        l as i32
    }
}