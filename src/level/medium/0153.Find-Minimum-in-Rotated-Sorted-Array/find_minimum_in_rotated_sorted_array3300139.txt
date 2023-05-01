// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/3300139/rust-binary-search/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len()-1);
        while l < r {
            let mid = (l + r)/2;
            if nums[r] < nums[mid] {
                l = mid + 1;
            } else { 
                r = mid;
            }
        }
        nums[l]
    }
}