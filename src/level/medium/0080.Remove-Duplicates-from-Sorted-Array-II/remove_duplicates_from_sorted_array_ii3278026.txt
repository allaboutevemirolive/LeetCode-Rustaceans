// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/3278026/clean-o-n-rust-solution/
impl Solution {
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return nums.len() as i32;
    }

    let mut k = 2;

    for i in 2..nums.len() {
        if nums[i] != nums[k-1] || nums[k-1] != nums[k-2] {
            nums[k] = nums[i];
            k += 1;
        } 
    }

    k as i32
}
}