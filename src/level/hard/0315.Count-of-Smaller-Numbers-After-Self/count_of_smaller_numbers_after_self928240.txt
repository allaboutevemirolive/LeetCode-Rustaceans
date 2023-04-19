// https://leetcode.com/problems/count-of-smaller-numbers-after-self/solutions/928240/rust-one-liner/
impl Solution {
	pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .map(|(i, e)| nums[i + 1..].iter().filter(|&x| x < e).count() as i32)
            .collect()
    }
}