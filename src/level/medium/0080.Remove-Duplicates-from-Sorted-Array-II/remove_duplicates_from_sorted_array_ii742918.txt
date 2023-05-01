// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/742918/rust-cheapest-best/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        match nums.len() {
            0 | 1 => nums.len() as i32,
            _ => (2..nums.len()).fold(2i32, |mut k, i| {
                if nums[(k - 2) as usize] != nums[i] {
                    nums[k as usize] = nums[i];
                    k += 1;
                }
                k
            }),
        }
    }
}