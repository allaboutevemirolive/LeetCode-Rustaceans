// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/968274/rust-onepass/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut num_dist = std::collections::HashMap::new();
        let mut res = 0;
        for cur in 0..nums.len() {
            let num = nums[cur];
            let appeared = num_dist.entry(num).or_insert(0);
            if *appeared < 2 {
                *appeared += 1;
                nums[res] = num;
                res += 1;
            }
        }
        res as i32
    }
}