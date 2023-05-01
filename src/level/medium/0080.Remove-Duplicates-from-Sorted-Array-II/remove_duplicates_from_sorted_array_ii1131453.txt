// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/1131453/rust-0ms/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        
        for n in 0..nums.len() {
            let cur = nums[n];
            if i < 2 || cur > nums[i-2] {
                nums[i] = cur;
                i += 1;
            }
        }
        
        i as i32
    }
}