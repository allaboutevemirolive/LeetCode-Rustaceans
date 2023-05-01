// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/314117/simple-rust/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }
    let mut tar = 2; 
    for i in 2..nums.len() {
        if nums[tar-2] != nums[i] {
            nums[tar] = nums[i];
            tar += 1;
        }
    }
    return tar as i32;
}