// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/717324/rust-solutions/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 2;

        while i < nums.len() {
            if nums[i] == nums[i - 2] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}