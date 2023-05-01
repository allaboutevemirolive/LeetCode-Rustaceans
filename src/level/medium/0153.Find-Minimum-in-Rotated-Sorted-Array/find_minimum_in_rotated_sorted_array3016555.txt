// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/3016555/rust-binary-search-solution/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32 - 1);

        while l < r {
            let mid = l + (r - l) / 2;

            if nums[mid as usize] < nums[r as usize] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        nums[l as usize]
    }
}