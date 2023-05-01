// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/3352635/rust-solution/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums[0] < nums[nums.len() - 1] {
            return nums[0];
        }
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid] < nums[0] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        nums[lo]        
    }
}