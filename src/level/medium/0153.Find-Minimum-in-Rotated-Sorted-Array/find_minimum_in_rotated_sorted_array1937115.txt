// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/1937115/rust-binary-search/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l : usize = 1; 
        let mut h : usize = nums.len();
        let n = nums.len();
        while (l < h) {
            let i = (l + h)/2;
            if (nums[i] > nums[0]) {
                l = i+1;
            }
            else { 
                h = i;
            }
        }
        nums[l % n] as i32
    }
}