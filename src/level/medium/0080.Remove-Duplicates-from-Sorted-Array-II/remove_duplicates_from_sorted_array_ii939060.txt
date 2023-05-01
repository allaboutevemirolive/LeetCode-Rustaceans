// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/939060/rust-linear-solution/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        match nums.len() {
            0 | 1 => nums.len() as i32,
            _ => {
                let mut ptr = 2;
                for i in ptr..nums.len() {
                    if nums[ptr-2] != nums[i] {
                        nums[ptr] = nums[i];
                        ptr += 1;
                    }
                }
                ptr as i32
            }
        }
    }
}